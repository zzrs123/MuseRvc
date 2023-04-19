use std::env;

fn main() {
    // 从命令行参数中获取传入的参数
    let args: Vec<String> = env::args().collect();

    // 判断传入程序的参数是否为2个，args[0]为程序名称，args[1]为传入的第一个参数
    if args.len() != 2 {
        // 异常处理，提示参数数量不对。
        // eprintln，向标准错误流输出字符串
        eprintln!("{}: invalid number of arguments", args[0]);
        // 程序返回值不为0时，表示存在错误
        std::process::exit(1);
    }

    // 声明一个全局main段，同时也是程序入口段
    println!("  .globl main");
    // main段标签
    println!("main:");
    // li为addi别名指令，加载一个立即数到寄存器中
    // 传入程序的参数为str类型，因为需要转换为需要int类型，
    // parse为将字符串类型转换为int类型的方法
    println!("  li a0, {}", args[1].parse::<i32>().unwrap());
    // ret为jalr x0, x1, 0别名指令，用于返回子程序
    println!("  ret");
}