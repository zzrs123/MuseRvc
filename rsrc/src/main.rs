use std::env;
use std::process::*;
// use std::str::FromStr;

/*=====================================================================
                    Token系统
// ====================================================================== */
#[derive(PartialEq)]
enum TokenKind {
    TkPunct, // 操作符如： + -
    TkNum,   // 数字
    TKEof,   // 文件终止符，即文件的最后
}

enum V<'a> {
    Int(i32),
    Str(&'a str),
}

struct Token<'a> {
    kind: TokenKind,
    value: Option<V<'a>>,
    len: usize,
}

/*=====================================================================
    error 宏
    在这个宏中，用 $fmt 作为必需的参数。
    然后使用 $arg 变量来捕获任意数量的额外参数。
    使用 eprint! 和 \n 字符将消息输出到 stderr 流中
 ======================================================================*/ 
macro_rules! error {
    ($fmt:expr $(, $arg:expr)*) => {{
        eprint!(concat!($fmt, "\n") $(, $arg)*);
        exit(1);
    }};
}

/*=====================================================================
    get_token_number
    返回 TKNUM 的值
 ======================================================================*/ 

fn get_token_number(token: Option<&Token>) -> i32 {

    if let Some(v)= token{
        if v.kind != TokenKind::TkNum{
            error!("expected a number not a char|string")
        } else if let Some(V::Int(n)) = v.value{
            return n;
        }
    }
    error!("expect a number");
}

/*======================================================================
                    equal: 字符类型/操作符匹配函数
                比较Token的字符value与传入的参数字符串是否相等
                            返回布尔值
====================================================================== */
fn equal(token: &Token, s: &str) -> bool {

    if token.len != s.len(){
        return false;
    } else if let Some(V::Str(st)) = token.value{
        let judge = s == st;
        return judge;
    }
    false
}

/*======================================================================
                    toknize: Token解析主干函数
            从头到尾扫描args[1]，对不同类型的token做不同处理
                rust用match+vec<T>实现起来是相当优雅的
====================================================================== */
fn tokenize(arg: &mut str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut start = 0;
    // arg.char_indices()同时得到索引和字符，对得到的字符用match进行处理
    for (i, c) in arg.char_indices() {
        match c {
            // 处理空白字符
            c if c.is_whitespace() => {
                start = i + 1;
            },
            // 解析操作符
            // 特点是长度一定为1
            '+' | '-' => {
                let str1=&arg[start..=i] ;
                let token = Token {
                    kind: TokenKind::TkPunct,
                    value: Some(V::Str(str1)),
                    // loc: &arg[start..i],
                    len: 1, // 操作符长度为1
                    // sss: Some(str1), // 将操作符解析到 sss 字段
                };
                tokens.push(token);
                start = i + 1;
            },
            // 解析数字
            '0'..='9' =>  {
                let mut end = i;
                while let Some(c) = arg.chars().nth(end) {
                    if c.is_digit(10) {
                        end += 1;
                    } else {
                        break;
                    }
                }
                let numeric = arg[start..end].parse::<i32>().ok();
                let token = Token {
                    kind: TokenKind::TkNum,
                    value: Some(V::Int(numeric.unwrap())),
                    // loc: &arg[start..end],
                    len: end - start,
                    // sss:None
                };
                tokens.push(token);
                start = end;
            }
            _ => {
                error!("Unexpected character '{}'", c);
            }
        }
    }
    let eof_token = Token {
        kind: TokenKind::TKEof,
        value: None,
        // loc: &arg[start..],
        len: arg.len() - start,
        // sss:None,
    };
    tokens.push(eof_token);
    tokens
}

/*======================================================================
                        main: 主干函数
                接收目标字符串，接入Token处理为RISC-V汇编
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
  
    // 接下来引入Token解析系统，将数字和运算符处理为token，空格滤除
    let tok = tokenize(args[1].as_mut_str());

    // let mut iter = tok.chars();//创建了一个字符迭代器
    // let mut p = iter.next();//获取其第一个字符
    let mut iter = tok.iter();// 创建了一个迭代器

    let mut p = iter.next();//获取其第一个token
    // 声明全局main段，也是程序入口段
    println!("  .globl main");
  
    // main段标签
    println!("main:");

    // li为addi别名指令，加载一个立即数到寄存器中
    // 这里我们将算式分解为 num (op num) (op num)... 的形式
    // 所以先将第一个 num 传入a0
    let c = p.unwrap();
    let num = get_token_number(Some(c));
    println!("  li a0, {}", num);

    while let Some(op) = iter.next() {
        // println!("{}",op.loc);
        match op.kind{
            TokenKind::TKEof    => {
                break // 这里不能error！，直接退出不能打印ret
            }
            TokenKind::TkPunct  => {
                if equal(op, "+"){
                    p = iter.next(); // 跳过 + 号
                    println!("  addi a0, a0, {}", get_token_number(Some(p.unwrap())));
                } else {
                    p = iter.next(); // 跳过 - 号
                    println!("  addi a0, a0, -{}", get_token_number(Some(p.unwrap())));
                }
            
            }
            TokenKind::TkNum    => {
                error!("unexpected num!")
            }
        }
    }
    println!("  ret");
}

