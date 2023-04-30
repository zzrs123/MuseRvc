mod lexer;
mod parser;
mod ast;
mod error;
mod codegen;

use std::env;
// use std::process::*;
// use std::result::Result;
use parser::parser;
use codegen::gen_asm;
use codegen::DEPTH;
use lexer::tokenize;
use error::*;

/*======================================================================
                        main: 主干函数
                接收目标字符串，接入Token处理为RISC-V汇编
                string -> token -> ast-node -> asm
====================================================================== */
fn main() {
    // 从命令行参数中获取传入的参数
    let mut args: Vec<String> = env::args().collect();

     // 判断传入程序的参数是否为2个，args[0]为程序名称，args[1]为传入的第一个参数
    if args.len() != 2 {
        // 异常处理，提示参数数量不对。
        // 封装为error宏
        error!("{}: invalid number of arguments", &args[0]);
    }
    
    //---------------- 错误处理系统 ---------------
    let total_str = args[1].clone();
    unsafe {
        CURRENT_INPUT = Some(total_str);
    }

    // ------------------ 词法分析 ----------------------
    // 接下来引入Token解析系统，将数字和运算符处理为token，空格滤除
    let tok = tokenize(args[1].as_mut_str());
    // VerrorAt(1,"tskldjf");
    // number: 1
    // operator: -
    // operator: -
    // number: 1
    // unknown value

    /*==========================================================
                        词法单元测试模块
                每次新增功能都要先保证词法分析正确
     ===========================================================*/
    // for token in tok.iter() {
    //     match &token.value {
    //         Some(V::Int(num)) => println!("number: {}", num),
    //         Some(V::Str(s)) => println!("operator: {}", s),
    //         _ => println!("unknown value"),
    //     }
    // }
    // ------------------ 文法分析 -----------------------
    let node= match parser(&tok) {
        Ok(getit) => getit,
        Err(_) => return ()
        // Err(err) =>  Err(err),
    };

    // let mut iter = tok.chars();//创建了一个字符迭代器
    // let mut p = iter.next();//获取其第一个字符
    // let mut iter = tok.iter();// 创建了一个迭代器
    // let mut p = iter.next();//获取其第一个token
    // 声明一个全局main段，同时也是程序入口段
    println!("  .globl main");
    // main段标签
    println!("main:");  
    // ------------------ 代码生成 -----------------------
    gen_asm(node);

    // ret为jalr x0, x1, 0别名指令，用于返回子程序
    // 返回的为a0的值
    println!("ret");
    // 增加一层检查：如果栈未清空则报错
    unsafe{
        assert!(DEPTH==0);
    }
}