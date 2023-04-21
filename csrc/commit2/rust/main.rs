use std::env;
use std::process::*;
// use std::str::FromStr;
fn main() {
    // 从命令行参数中获取传入的参数
    let args: Vec<String> = env::args().collect();

        // 判断传入程序的参数是否为2个，args[0]为程序名称，args[1]为传入的第一个参数
        if args.len() != 2 {
            // 异常处理，提示参数数量不对。
            // eprintln，向标准错误流输出字符串
            eprintln!("{}: invalid number of arguments", args[0]);
            // 程序返回值不为0时，表示存在错误
            exit(1);
    }

    let opera = args[1].as_str();

    let mut iter = opera.chars();//创建了一个字符迭代器
    let mut p = iter.next();//获取其第一个字符

    println!(" .globl main");
    println!("main:");
    let c = p.unwrap();

    println!("  li a0, {}",c);
  
    while let Some(op) = iter.next() {
  
        match op {
            '+' => {
                p=iter.next();//跳过+号
                println!("  addi a0, a0, {}", p.unwrap());
            }   
            '-' => {
                p=iter.next();//跳过-号
                println!("  addi a0, a0, -{}", p.unwrap());
                // println!("  add a0, a0, t0");
            }
            _ => panic!("unexpected operator"),
        }
    }
    println!("  ret");
}

