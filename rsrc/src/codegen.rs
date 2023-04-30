use crate::ast::*;
use crate::error;
/*====================================================================
                        代码生成模块 
                         栈管理函数
 ====================================================================*/

// 记录栈深度
pub(crate) static mut DEPTH: i32 = 0;

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

/*======================================================================
                        GenAsm: 代码生成主干函数
                接收目标字符串，接入Token处理为RISC-V汇编
====================================================================== */
pub fn gen_asm(nd:Box<Node> ){

    // step5中开头不一定是数字了，也可能是±.
    match nd.kind{
         // 加载数字到a0
        NodeKind::NdNum => { 
            println!("   li a0, {}", nd.val.unwrap()); 
            return;
        },
        NodeKind::NdNeg => {
            gen_asm(nd.lhs.unwrap());
            // neg a0, a0是sub a0, x0, a0的别名, 即a0=0-a0
            println!("   neg a0, a0");
            return;
        },
        _ => {
            // 递归到最右节点
            gen_asm(nd.rhs.unwrap());
            // 将结果压入栈
            push();
            // 递归到左节点
            gen_asm(nd.lhs.unwrap());
            // 将结果弹栈到a1
            pop("a1");
            // 生成各个二叉树节点
            match nd.kind {
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
                // a0=a0^a1，异或指令
                NodeKind::NdEq | NodeKind::NdNeq => {
                    println!("  xor a0, a0, a1");
                    // a0==a1 a0=a0^a1, sltiu a0, a0, 1 等于0则置1
                    // a0!=a1 a0=a0^a1, sltu a0, x0, a0 不等于0则置1
                    if nd.kind == NodeKind::NdEq {
                        println!("  seqz a0, a0");
                    } else {
                        println!("  snez a0, a0");
                    }
                    return;
                }
                NodeKind::NdLt => {
                    println!("  slt a0, a0, a1");
                    return;
                }
                // a0<=a1等价于
                // a0=a1<a0, a0=a1^1
                NodeKind::NdLe => {
                    println!("  slt a0, a1, a0");
                    println!("  xori a0, a0, 1");
                    return;
                }     
                   
                _ => {
                    error!("invalid expression");
                }
            }
        }
    }
}