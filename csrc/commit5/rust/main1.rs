// 第一版实现
use std::env;
use std::process::*;
// use std::ptr::eq;
// use std::rc::Rc;
use std::result::Result;
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
    loc: usize,
}
static mut CURRENT_INPUT : Option<String> = None;
/*=====================================================================
                        verror_at
                    指示错误位置，输出报错消息
 ======================================================================*/ 

fn verror_at(loc:usize, err_str:&str) -> (){
    unsafe{
        eprintln!("{}", CURRENT_INPUT.as_deref().unwrap()); // 输出源串
    }
    // 根据loc找到出错位置，另起一行用^+报错消息输出
    let padding = " ".repeat(loc);
    eprint!("{}",padding);
    eprint!("{}", '^');
    eprint!("{}\n", err_str);
}

 /*=====================================================================
                error 宏
    尝试了很多方式，最后还是返璞归真复用error!宏
    在前面的error版本中直接向下增加就可以，但总感觉写的不够好。
    因为C中使用变参函数，Rust不支持，所以寻求使用宏匹配来解决
    跟C有点不同，C中区分了两种错误消息宏
    第一种处理：字符串解析为token过程中的errorAT，
              这个使用error宏的第一个匹配逻辑就可以达到同样效果
    第二种处理：解析token流时的错误errorTok，
              使用输入为token类型的第二个匹配逻辑
 ======================================================================*/ 
macro_rules! error {
    ($tok:ident, $msg:expr) => {
        verror_at($tok.loc, $msg);
        exit(1);
    };
    // 使用 format! 宏将 $c 的值插入到 $fmt 中，生成新的字符串 message。
    // 最后，我们将 $i 和 &message 一起传递给 verror_at 函数进行错误处理。
    ($i:ident, $fmt:expr, $c:expr) =>{{
        // let str = concat!($fmt, $c:expr) ;
        let message = format!($fmt, $c);
        verror_at($i,&message);
        exit(1);
    }};
    ($fmt:expr $(, $arg:expr)*) => {{
        eprint!(concat!($fmt, "\n") $(, $arg)*);
        exit(1);
    }};

}
 /*=====================================================================
    errorTok 宏
    将token解析错误的位置标注出来进行输出。
    这里跟C有点不同，C中区分了两种错误消息宏
    第一种处理：字符串解析为token过程中的errorAT，这个使用error宏就可以达到同样效果
    第二种处理：解析token流时的错误errorTok，单独实现了一个输入为tok的一个宏
 ======================================================================*/ 
// macro_rules! errorTok{ 
//     ($tok:ident $arg:expr) => {
//         verror_at($tok.loc,$arg);
//         exit(1);
//     };
// }


/*=====================================================================
    get_token_number
    返回 TKNUM 的值
 ======================================================================*/ 

// fn get_token_number(token: Option<&Token>) -> i32 {

//     if let Some(v)= token{
//         if v.kind != TokenKind::TkNum{
//             // error!("expected a number not a char|string")
//             error!(v,"expected a number instead of a char|string|EOF");
//         } else if let Some(V::Int(n)) = v.value{
//             return n;
//         }
//         error!(v,"expect a number");
//     }
//     error!("unknown error");
// }

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
                    skip: 跳过指定字符、字符串
====================================================================== */
// fn skip(rest: &mut Token, ex_str: &str,) {
//     if !equal(rest, ex_str) {
//         let arg1 = rest.loc;
//         error!(arg1, "expect '{}'", ex_str);
//     }

//     // rest.next = Some(rest.next.unwrap().clone());
// }

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
            // 改用函数识别操作符
            c if c.is_ascii_punctuation() => {
                let str1=&arg[start..=i] ;
                let token = Token {
                    kind: TokenKind::TkPunct,
                    value: Some(V::Str(str1)),
                    loc: i,
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
                    loc: i,
                    len: end - start,
                    // sss:None
                };
                tokens.push(token);
                start = end;
            }
            _ => {
                // 处理无法识别的字符
                let loc_char: usize = i;
                error!(loc_char,"Unexpected character '{}'", c);
                // errorTok!()
            }
        }
    }
    let eof_token = Token {
        kind: TokenKind::TKEof,
        value: None,
        loc: arg.len(),
        len: 0,
        // sss:None,
    };
    tokens.push(eof_token);
    tokens
}


/*======================================================================
                    AST 抽象语法树 系统 == 语法解析
 =======================================================================*/
#[derive(PartialEq)]
// AST的节点种类
enum NodeKind {
    NdAdd, // +
    NdSub, // -
    NdMul, // *
    NdDiv, // /
    NdNum, // 整型
}
  
  // AST中二叉树节点
struct Node {
    kind: NodeKind, // 节点种类
    lhs: Option<Box<Node>>, // 左部，left-hand side
    rhs: Option<Box<Node>>, // 右部，right-hand side
    val: Option<i32>, // 存储ND_NUM种类的值，不需要存储操作符（类型判断）
}
impl Node {
    fn new_node(kind: NodeKind) -> Box<Node> {
        Box::new(Node {
            kind,
            lhs: None,
            rhs: None,
            val: None,
        })
    }

    fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        let mut node = Node::new_node(kind);
        node.lhs = Some(lhs);
        node.rhs = Some(rhs);
        node
    }

    fn new_num(val: i32) -> Box<Node> {
        let mut node = Node::new_node(NodeKind::NdNum);
        node.val = Some(val);
        node
    }
}

/*====================================================================
                        paser - 生成AST的逻辑顶层
                    主要负责实现 Token vec 与 二叉树 的连接
 ====================================================================*/
fn paser(tokens: &Vec<Token>) -> Result<Box<Node>, String> {
    let mut pos = 0;
    // let expression = expr(tokens, &mut pos)?;
    let expression = expr(tokens, &mut pos)?;
    if pos != tokens.len()- 1 {
        return Err(String::from("Unexpected tokens"));
        // error!("Unexpected tokens")
    }
    Ok(expression)
    // expression
}
/*====================================================================
                        expr - 生成AST的实际顶层
                expr = mul ("+" mul | "-" mul)* 
                    表达式是由多个乘数相加得到的。
 ====================================================================*/
 fn expr(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    // mul
    let mut node = mul(tokens, pos)?;

    // ("+" mul | "-" mul)*
    loop {
        let tmp = tokens.get(*pos);
        match  tmp{
            tmp if equal(tmp.unwrap(),"+") => {
                *pos += 1;
                let right = mul(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdAdd,node,right);
            }
            tmp if equal(tmp.unwrap(),"-") => {
                *pos += 1;
                let right = mul(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdSub,node,right);
            }
            _ => { break; }
        }
    }
    Ok(node)
}


/*====================================================================
                        mul - 乘数节点生成模块
            mul = primary ("*" primary | "/" primary)*
                    乘数本身是由基数相乘或相除得到的
 ====================================================================*/
 fn mul(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    let mut node = primary(tokens, pos)?;
    loop {
        let tmp = tokens.get(*pos); 
        match tmp {
            tmp if equal(tmp.unwrap(),"*")=> {
                *pos += 1;
                let right = primary(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdMul, node, right);
            }
            tmp if equal(tmp.unwrap(),"/") => {
                *pos += 1;
                let right = primary(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdDiv, node, right);
            }
            _ => { break; }
        }
    }
    Ok(node)
}
/*====================================================================
                        primary - 基数节点生成模块
                      primary = "(" expr ")" | num
                      基数是一个括号内的表达式或者一个数字
 ====================================================================*/
 fn primary(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    // "(" expr ")"
    let tmp = tokens.get(*pos);
    match tmp {
        tmp if equal(tmp.unwrap(), "(") => {
            *pos += 1;
            let node = expr(tokens, pos)?;
            if equal(tokens.get(*pos).unwrap(),")" ){
                *pos += 1;
                Ok(node)
            } else {
                error!("Missing closing paren: ')'")
            }
        }
        tmp if tmp.unwrap().kind == TokenKind::TkNum =>{
            *pos += 1;
            let mut nnn = 0;
            if let Some(V::Int(num)) = tmp.unwrap().value{
                nnn = num;
            }
            let node = Node::new_num(nnn);
            Ok(node)
        }
        _ => {
            error!("Expected factor: '*' | '/'");
        }
    }
    
}





// 记录栈深度
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

// 生成表达式


/*======================================================================
                        GenAsm: 代码生成主干函数
                接收目标字符串，接入Token处理为RISC-V汇编
====================================================================== */
fn gen_asm(nd:Box<Node> ){
    // 加载数字到a0
    if nd.kind == NodeKind::NdNum {
        println!("   li a0, {}", nd.val.unwrap());
        return;
    }

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
        NodeKind::NdAdd => { // + a0=a0+a1
            println!("   add a0, a0, a1");
        },
        NodeKind::NdSub => { // - a0=a0-a1
            println!("   sub a0, a0, a1");
        },
        NodeKind::NdMul => { // * a0=a0*a1
            println!("   mul a0, a0, a1");
        },
        NodeKind::NdDiv => { // / a0=a0/a1
            println!("   div a0, a0, a1");
        },
        _ => {
            error!("invalid expression");
        }
    }

}


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

    // ------------------ 文法分析 -----------------------
    let node= match paser(&tok) {
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
}

