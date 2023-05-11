/*====================================================================
                         代码生成模块 
 ====================================================================*/
use crate::ast::*;
use crate::error;
/*====================================================================
                         栈管理函数
 ====================================================================*/

// 记录栈深度 -- 后期可以考虑改成结构体成员
static mut DEPTH: i32 = 0;

// 压栈，将结果临时压入栈中备用
// sp为栈指针，栈反向向下增长，64位下，8个字节为一个单位，所以sp-8
// 当前栈指针的地址就是sp，将a0的值压入栈
// 不使用寄存器存储的原因是因为需要存储的值的数量是变化的。
fn push() {
    println!("   addi sp, sp, -8");
    println!("   sd a0, 0(sp)");
    unsafe {
        DEPTH += 1;
    }
}

// 弹栈，将sp指向的地址的值，弹出到a1
fn pop(reg: &str) {
    println!("   ld {}, 0(sp)", reg);
    println!("   addi sp, sp, 8");
    unsafe {
        DEPTH -= 1;
    }
}


/// 将 N 对齐到 Align 的整数倍
fn align_to(n: i32, align: i32) -> i32 {
    // if align <= 0 {
    //     return n;
    // }
    // (0, Align] 返回 Align
    (n + align - 1) / align * align
}

// 权宜之计，遍历二叉树（感觉效率好低，要重复修改locals）
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
}
/*====================================================================
                    assign_lvar_offsets：设置变量栈
 ====================================================================*/
fn assign_lvar_offsets(mut prog: Function) -> Function {
    let mut offset = 0;

    for var in prog.locals.iter_mut() {
        offset += 8;
        var.offset = -offset;
    }
    // 遍历函数体中的语法树，同步更新变量的偏移量
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
            println!("   addi a0, fp, {}", node.var.unwrap().offset);
        }
        _ => {
            error!("not an lvalue");
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
            println!("   li a0, {}", node.val.unwrap());
        }
        NodeKind::NdNeg => {
            gen_expr(node.lhs.unwrap());
            println!("   neg a0, a0");
        }
        NodeKind::NdVar => {
            gen_addr(node);
            println!("   ld a0, 0(a0)");
        }
        NodeKind::NdAssign => {
            // 左部是左值，保存值到的地址
            gen_addr(node.lhs.unwrap());
            push();
            // 右部是右值，为表达式的值
            gen_expr(node.rhs.unwrap());
            pop("a1");
            println!("   sd a0, 0(a1)");
        }
        _ => {
            gen_expr(node.rhs.unwrap());
            push();
            gen_expr(node.lhs.unwrap());
            pop("a1");
            match node.kind {
                // + a0=a0+a1
                NodeKind::NdAdd => { 
                    println!("   add a0, a0, a1");
                    return;
                },
                // - a0=a0-a1
                NodeKind::NdSub => { 
                    println!("   sub a0, a0, a1");
                    return;
                },
                // * a0=a0*a1
                NodeKind::NdMul => { 
                    println!("   mul a0, a0, a1");
                    return;
                },
                // / a0=a0/a1
                NodeKind::NdDiv => {
                    println!("   div a0, a0, a1");
                    return;
                },
                NodeKind::NdEq | NodeKind::NdNeq => {
                    // a0=a0^a1，异或指令
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
                    println!("   slt a0, a0, a1");
                    return;
                }
                // a0<=a1等价于
                // a0=a1<a0, a0=a1^1
                NodeKind::NdLe => {
                    println!("   slt a0, a1, a0");
                    println!("   xori a0, a0, 1");
                    return;
                }     
                   
                _ => {
                    error!("invalid expression");
                }
            }
        }
    }
}

    
    
/*======================================================================
                        gen_stmt: 生成语句
                    处理文法分析的语句节点，处理语句逻辑
====================================================================== */

fn gen_stmt(node:Box<Node>) {

    // if node.kind == NodeKind::NdExprStmt {
    //     gen_expr(node.lhs.unwrap());
    //     return;
    // }
    match node.kind {
        NodeKind::NdExprStmt => {
            gen_expr(node.lhs.unwrap());
            return; 
        }
        NodeKind::NdReturn  => {
            gen_expr(node.lhs.unwrap());
            println!("   j .L.return");
        }
        _ => {
            error!("invalid statement");
        }
    }
    
}


/*======================================================================
                        codegen: 代码生成入口函数
                            包含代码块的基础信息
====================================================================== */
pub fn codegen(prog: Function) {
    let prog_pros = assign_lvar_offsets(prog);
    // for var in prog_pros.locals.iter() {
    //     println!("Var offset: {}", var.offset);
    // }
    // Var offset: -8
    // Var offset: -16
    println!(".globl main");
    println!("main:");

    //==============栈布局==============(从这里也可以看到变量的类型并不丰富，每个变量8字节)
    //-------------------------------// sp
    //              fp
    //-------------------------------// fp = sp-8
    //             变量
    //-------------------------------// sp = sp-8-StackSize
    //           表达式计算
    //-------------------------------//
    //==============栈布局==============

    
    // Prologue, 前言
    // 将fp压入栈中，保存fp的值
    println!("   addi sp, sp, -8");
    println!("   sd fp, 0(sp)");
    // 将sp写入fp
    println!("   mv fp, sp");
    // 26个字母*8字节=208字节，栈腾出208字节的空间
    // println!("   addi sp, sp, -208");
    println!("   addi sp, sp, -{}", prog_pros.stack_size);
    let nd = prog_pros.body.unwrap();
    let mut n = Some(nd);
    while let Some(node) = n {
        gen_stmt(node.clone());
        unsafe {
            assert_eq!(DEPTH, 0);
        }
        n = node.next.clone();
    }
    // Epilogue，后语
    // 输出return标签
    println!(".L.return:");
    // 将fp的值改写回sp
    println!("   mv sp, fp");
    // 将最早fp保存的值弹栈，恢复fp。
    println!("   ld fp, 0(sp)");
    println!("   addi sp, sp, 8");
    // 返回
    println!("ret");
}

