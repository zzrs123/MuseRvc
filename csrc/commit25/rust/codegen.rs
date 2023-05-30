/*====================================================================
                         代码生成模块 
 ====================================================================*/
use crate::ast::*;
use crate::error;
use crate::error::verror_at;


// 记录栈深度 -- 后期可以考虑改成结构体成员
static mut DEPTH: i32 = 0;
// 记录函数调用使用的寄存器
static mut ARG_REG_LIST: Vec<&str> = vec![];
// 记录当前函数名
static mut CURRENT_FUNC : Option<String> = None;

/*====================================================================
                         栈管理函数
 ====================================================================*/
// 压栈，将结果临时压入栈中备用
// sp为栈指针，栈反向向下增长，64位下，8个字节为一个单位，所以sp-8
// 当前栈指针的地址就是sp，将a0的值压入栈
// 不使用寄存器存储的原因是因为需要存储的值的数量是变化的。
fn push() {
    println!("   # 压栈,将a0的值存入栈顶");
    println!("   addi sp, sp, -8");
    println!("   sd a0, 0(sp)");
    unsafe {
        DEPTH += 1;
    }
}

// 弹栈，将sp指向的地址的值，弹出到a1
fn pop(reg: &str) {
    println!("   # 弹栈，将栈顶的值存入{}", reg);
    println!("   ld {}, 0(sp)", reg);
    println!("   addi sp, sp, 8");
    unsafe {
        DEPTH -= 1;
    }
}

/*====================================================================
                align_to：将 N 对齐到 Align 的整数倍
 ====================================================================*/

fn align_to(n: i32, align: i32) -> i32 {
    // if align <= 0 {
    //     return n;
    // }
    // (0, Align] 返回 Align
    (n + align - 1) / align * align
}

/*====================================================================
                sync_node_offsets：遍历二叉树修正var-offset
                    权宜之计。感觉效率好低，要重复修改locals
 ====================================================================*/

fn sync_node_offsets(node: &mut Node, locals: &Vec<Obj>) {
    match node.kind {
        NodeKind::NdVar => {
            let name = node.name.clone().unwrap();
            if let Some(var) = locals.iter().find(|&v| v.name == name) {
                node.var = Some(var.clone());
            }
        }
        _ => {}
    }

    if let Some(ref mut lhs) = node.lhs {
        sync_node_offsets(lhs.as_mut(), locals);
    }
    if let Some(ref mut rhs) = node.rhs {
        sync_node_offsets(rhs.as_mut(), locals);
    }
    if let Some(ref mut next) = node.next {
        sync_node_offsets(next.as_mut(), locals);
    }
    // step12最困惑的一个bug，这让我感觉自己设计的数据结构很冗余、重复、复杂
    // 具体说来是，step11之前只需要前三个if let，而由于step12的设计，需要在二叉树中继续引入body分支
    if let Some(ref mut next) = node.body {
        sync_node_offsets(next.as_mut(), locals);
    }
    if let Some(ref mut next) = node.init {
        sync_node_offsets(next.as_mut(), locals);
    }
    if let Some(ref mut next) = node.inc {
        sync_node_offsets(next.as_mut(), locals);
    }
    if let Some(ref mut next) = node.cond {
        sync_node_offsets(next.as_mut(), locals);
    }
    if let Some(ref mut next) = node.then {
        sync_node_offsets(next.as_mut(), locals);
    }
    if let Some(ref mut next) = node.els {
        sync_node_offsets(next.as_mut(), locals);
    }
}

/*====================================================================
                    assign_lvar_offsets：设置变量栈
            step19：发现惊天大问题，应当反向计算offset（增加了.rev()翻转列表）
 ====================================================================*/
fn assign_lvar_offsets(mut prog: Function) -> Function {
    let mut offset = 0;
    // println!("sdfasdfsdfasdfasdfasdf");
    for var in prog.locals.iter_mut().rev() {
        offset += 8;
        var.offset = -offset;
    }
    // 遍历函数体中的语法树，同步更新变量的偏移量（这一步感觉很麻烦）
    sync_node_offsets(prog.body.as_mut().unwrap(), &prog.locals);
    prog.stack_size = align_to(offset, 16);

    prog
}

/*======================================================================
                    gen_addr: 计算给定节点的绝对地址
                        如果报错，说明节点不在内存中
====================================================================== */
fn gen_addr(node: Box<Node>) {
    match node.kind {
        NodeKind::NdVar => {
            // 取出第一个字符并转换为 u8 类型
            // let name = node.name.as_deref().unwrap_or_default().as_bytes()[0];
            // 偏移量 = 是两个字母在 ASCII 码表中的距离加 1 后乘以 8，*8 表示每个变量需要 8 个字节单位的内存
            // 结合CodeGen的栈布局进行理解
            // let offset = (name - b'a' + 1) as i32 * 8;
            // println!("{}",node.clone().var.unwrap().offset);
            println!("   # 获取变量{}的栈内地址为{}(fp)", node.clone().var.unwrap().name, node.clone().var.unwrap().offset);
            println!("   addi a0, fp, {}", node.var.unwrap().offset);
        }
        // 解引用*
        NodeKind::NdDeref => {
            gen_expr(node.lhs.unwrap());
            return;
        }
        // 更新错误消息系统，还有待继续完善
        _ => {
            let loc: usize = node.tokloc;
            let name = node.name.unwrap();
            error!(loc,"{} not an lvalue", name);
        }
    }
}

/*======================================================================
                        GenExpr: 生成表达式
                    处理表达式，现在不是主干函数了
====================================================================== */
fn gen_expr(node: Box<Node>) {
    match node.kind {
        NodeKind::NdNum => {
            println!("   # 将{}加载到a0中", node.clone().val.unwrap());
            println!("   li a0, {}", node.val.unwrap());
        }
        NodeKind::NdNeg => {
            gen_expr(node.lhs.unwrap());
            println!("   # 对a0值进行取反");
            println!("   neg a0, a0");
        }
        NodeKind::NdVar => {
            gen_addr(node.clone());
            println!("   # 读取a0中存放的地址,得到的值存入a0");
            println!("   ld a0, 0(a0)");
        }
        // 解引用
        NodeKind::NdDeref  => {
            gen_expr(node.clone().lhs.unwrap());
            println!("   # 读取a0中存放的地址,得到的值存入a0");
            println!("   ld a0, 0(a0)");
            return;
        }
        // 取地址
        NodeKind::NdAddr   => {
            gen_addr(node.clone().lhs.unwrap());
            return;
        }
        // 赋值
        NodeKind::NdAssign => {
            // 左部是左值，保存值到的地址
            gen_addr(node.lhs.unwrap());
            push();
            // 右部是右值，为表达式的值
            gen_expr(node.rhs.unwrap());
            pop("a1");
            println!("   # 将a0的值,写入到a1中存放的地址");
            println!("   sd a0, 0(a1)");
        
        }
        NodeKind::NdFuncall => {
            let mut args_n = 0;
            let mut args = node.clone().args;
            while let Some(arg) = &args{
                gen_expr(arg.clone());
                push();
                args_n+=1;
                args = args.unwrap().next;
            }
            // 反向弹栈，a0->参数1，a1->参数2……
            for i in (0..args_n).rev() {
                pop(unsafe { ARG_REG_LIST[i] });
            }

            println!("\n   # 调用函数{}", node.funcname);
            println!("   call {}", node.funcname);
            return;
        }
       
        _ => {
            // 递归到最右节点
            gen_expr(node.rhs.unwrap());
            push();
            // 递归到左节点
            gen_expr(node.lhs.unwrap());
            pop("a1");
            match node.kind {
                // + a0=a0+a1
                NodeKind::NdAdd => { 
                    println!("   # a0+a1,结果写入a0");
                    println!("   add a0, a0, a1");
                    return;
                },
                // - a0=a0-a1
                NodeKind::NdSub => { 
                    println!("   # a0-a1,结果写入a0");
                    println!("   sub a0, a0, a1");
                    return;
                },
                // * a0=a0*a1
                NodeKind::NdMul => { 
                    println!("   # a0*a1,结果写入a0");
                    println!("   mul a0, a0, a1");
                    return;
                },
                // / a0=a0/a1
                NodeKind::NdDiv => {
                    println!("   # a0/a1,结果写入a0");
                    println!("   div a0, a0, a1");
                    return;
                },
                NodeKind::NdEq | NodeKind::NdNeq => {
                    // a0=a0^a1，异或指令
                    println!("   # 判断a0和a1的相等情况");
                    println!("   xor a0, a0, a1");
                    // a0==a1
                    // a0=a0^a1, sltiu a0, a0, 1
                    // 等于0则置1
                    // a0!=a1(else)
                    // a0=a0^a1, sltu a0, x0, a0
                    // 不等于0则置1
                    if node.kind == NodeKind::NdEq {
                        println!("   seqz a0, a0");
                    } else {
                        println!("   snez a0, a0");
                    }
                }
                NodeKind::NdLt => {
                    println!("   # 判断a0<a1");
                    println!("   slt a0, a0, a1");
                    return;
                }
                // a0<=a1等价于
                // a0=a1<a0, a0=a1^1
                NodeKind::NdLe => {
                    println!("   # 判断a0<=a1");
                    println!("   slt a0, a1, a0");
                    println!("   xori a0, a0, 1");
                    return;
                }     
                // 更新错误消息系统，还有待继续完善   
                _ => {
                    let loc = node.tokloc;
                    error!(loc,"invalid expression {}", "here");
                }
            }
        }
    }
}

/*======================================================================
                        count: 代码段计数
====================================================================== */
fn count() -> i32 {
    static mut I: i32 = 1;
    let count: i32;
    unsafe {
        count = I;
        I += 1;
    }
    count
}
    
/*======================================================================
                        gen_stmt: 生成语句
                    处理文法分析的语句节点，处理语句逻辑
====================================================================== */
fn gen_stmt(node:Box<Node>) {

    // 单独拿出来是为了提高效率（maybe）
    if let NodeKind::NdExprStmt = node.kind {
        gen_expr(node.lhs.unwrap()); 
        return;
    }
    match node.kind {
        NodeKind::NdBlock   => {
            let mut n = node.body;
            while let Some(node) = n {
                gen_stmt(node.clone());
                n = node.next;
            }
        }
        // 生成IF语句
        NodeKind::NdIf => {
            let c = count();
            println!("\n# =====分支语句{}==============", c);
            // 生成条件内语句
            println!("\n# Cond表达式{}", c);
            gen_expr(node.cond.unwrap());
            println!("  beqz a0, .L.else.{}", c);
            // 生成符合条件后的语句
            println!("\n# Then语句{}", c);
            gen_stmt(node.then.unwrap());
            // 执行完后跳转到if语句后面的语句
            println!("  # 跳转到分支{}的.L.end.{}段\n", c, c);
            println!("  j .L.end.{}", c);
            // else代码块，else可能为空，故输出标签
            println!("\n# Else语句{}", c);
            println!("# 分支{}的.L.else.{}段标签\n", c, c);
            println!(".L.else.{}:", c);
            // 生成不符合条件后的语句
            if let Some(els) = node.els {
                gen_stmt(els);
            }
            // 结束if语句，继续执行后面的语句
            println!("\n# 分支{}的.L.end.{}段标签", c, c);
            println!(".L.end.{}:", c);
        }
        // 生成for循环语句、while循环语句
        NodeKind::NdFor => {
            // 代码段计数
            let c = count();
            println!("\n# =====循环语句{}===============", c);
            // 生成初始化语句
            if let Some(init) = node.init {
                println!("\n# Init语句{}", c);
                gen_stmt(init);
            }
            // 输出循环头部标签
            println!("\n# 循环{}的.L.begin.{}段标签", c, c);
            println!(".L.begin.{}:", c);
            // 处理循环条件语句
            println!("# Cond表达式{}", c);
            if let Some(cond) = &node.cond {
                // 生成条件循环语句
                gen_expr(cond.clone());
                // 判断结果是否为0，为0则跳转到结束部分
                println!("   # 若a0为0,则跳转到循环{}的.L.end.{}段", c, c);
                println!("   beqz a0, .L.end.{}", c);
            }
            // 生成循环体语句
            println!("\n# Then语句{}", c);
            gen_stmt(node.then.unwrap());
            // 处理循环递增语句
            if let Some(inc) = &node.inc {
                // 生成循环递增语句
                println!("\n# Inc语句{}", c);
                gen_expr(inc.clone());
            }
            // 跳转到循环头部
            println!("   # 跳转到循环{}的.L.begin.{}段", c, c);
            println!("   j .L.begin.{}", c);
            // 输出循环尾部标签
            println!("\n# 循环{}的.L.end.{}段标签", c, c);
            println!(".L.end.{}:", c);
            return;
        }
        // NodeKind::NdExprStmt => {
        //     gen_expr(node.lhs.unwrap());
        //     return; 
        // }
        NodeKind::NdReturn  => {
            println!("# 返回语句");
            gen_expr(node.lhs.clone().unwrap());
            unsafe{
                println!("  # 跳转到.L.return.{}段", CURRENT_FUNC.clone().unwrap() );
                println!("  j .L.return.{}", CURRENT_FUNC.clone().unwrap());
            }
            
        }
         // 更新错误消息系统，还有待继续完善
        _ => {
            let loc: usize = node.tokloc;
            error!(loc, "invalid statement {}", "here");
        }
    }
    
}

/*======================================================================
                        init_regs: 设置寄存器
====================================================================== */
pub fn init_regs(){
    unsafe { 
        ARG_REG_LIST.push("a0");
        ARG_REG_LIST.push("a1");
        ARG_REG_LIST.push("a2");
        ARG_REG_LIST.push("a3");
        ARG_REG_LIST.push("a4");
        ARG_REG_LIST.push("a5");
    };
    
}

/*======================================================================
                        codegen: 代码生成入口函数
                            包含代码块的基础信息
====================================================================== */
pub fn codegen(mut prog: Program) {

    init_regs();
    
    for func in prog.funclist.iter_mut(){
        let prog_pros = assign_lvar_offsets(func.clone());
        unsafe { CURRENT_FUNC = Some(prog_pros.name.clone()) };
        
        println!("   #定义全局{}段", prog_pros.name);
        println!("   .globl {}", prog_pros.name);
        
        println!("# ====={}段开始===============\n", prog_pros.name);
        println!("# {}段标签",prog_pros.name);
        println!("{}:", prog_pros.name);
        //==============栈布局==============(从这里也可以看到变量的类型并不丰富，每个变量8字节)
        //-------------------------------// sp
        //              ra
        //-------------------------------// ra = sp-8
        //              fp
        //-------------------------------// fp = sp-16
        //             变量
        //-------------------------------// sp = sp-16-StackSize
        //           表达式计算
        //-------------------------------//
        //==============栈布局==============

         // Prologue, 前言
        // 将ra寄存器压栈,保存ra的值
        println!("  # 将ra寄存器压栈,保存ra的值");
        println!("  addi sp, sp, -16");
        println!("  sd ra, 8(sp)");
        // 再将fp压入栈中，保存fp的值
        println!("   # 将fp压栈,fp属于“被调用者保存”的寄存器,需要恢复原值");
        // println!("   addi sp, sp, -8");
        println!("   sd fp, 0(sp)");
        // 将sp写入fp
        println!("   # 将sp的值写入fp");
        println!("   mv fp, sp");
        
        // 偏移量为实际变量所用的栈大小
        println!("   # sp腾出StackSize大小的栈空间");
        println!("   addi sp, sp, -{}", prog_pros.stack_size);

        println!("\n# =====程序主体===============");
        gen_stmt(prog_pros.body.unwrap());

        // Epilogue，后语
        // 输出return标签
        println!("# ====={}段结束===============", prog_pros.name);
        println!("# return段标签");
        println!(".L.return.{}:", prog_pros.name);
        // 将fp的值改写回sp
        println!("   # 将fp的值写回sp");
        println!("   mv sp, fp");
        // 将最早fp保存的值弹栈，恢复fp。
        println!("   # 将最早fp保存的值弹栈，恢复fp和sp");
        println!("   ld fp, 0(sp)");
        // 将ra寄存器弹栈,恢复ra的值
        println!("   # 将ra寄存器弹栈,恢复ra的值");
        println!("   ld ra, 8(sp)");
        println!("   addi sp, sp, 16");
        // 返回
        println!("  # 返回a0值给系统调用");
        println!("  ret");
        
    }
    
}

