use std::env;
use std::process::*;
use std::result::Result;

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
#[allow(dead_code)]
struct Token<'a> {
    kind: TokenKind,
    value: Option<V<'a>>,
    loc: usize,
    len: usize,
}
/*=====================================================================
                            错误处理系统
                    CCURRENT_INPUT存储当前输入语句
// ====================================================================== */
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
            read_funct: 在tokenize函数中匹配操作符（长度为1或2）
                （因为没有通过测试用例而废弃）
====================================================================== */
// fn read_punct(ptr: &str) -> (Option<&str>, usize) {
//     let mut len = 0;
//     if let Some(c1) = ptr.chars().next() {
//         len += 1;
//         if let Some(c2) = ptr.chars().nth(1) {
//             len += 1;
//             match (c1, c2) {
//                 ('=', '=') | ('!', '=') | ('<', '=') | ('>', '=') => return (Some(&ptr[..2]), len),
//                 _ => if c1.is_ascii_punctuation() {
//                     return (Some(&ptr[..1]), 1);}
//             }
//         } 
//     }
//     (None, 0)
// }

/*======================================================================
                    toknize: Token解析主干函数
            从头到尾扫描args[1]，对不同类型的token做不同处理
                rust用match+vec<T>实现起来是相当优雅的
====================================================================== */
fn tokenize(arg: &mut str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut start = 0;
    // let mut iter = arg.char_indices().peekable();
    // arg.char_indices()同时得到索引和字符，对得到的字符用match进行处理
    for (i, c) in arg.char_indices() {
        if i < start {
            continue; //跳过已经匹配的字符
        }
        match c {
            // 处理空白字符
            c if c.is_whitespace() => {
                start = i + 1;
                continue;
            },
            // 解析操作符
            // 特点是长度一定为1或2
            c if c.is_ascii_punctuation() => {
                let mut punct_len = 1;
                if let Some(next_char) = arg.chars().nth(i + 1) {
                    if (c == '!' && next_char == '=') || (c == '=' && next_char == '=') || (c == '<' && next_char == '=') || (c == '>' && next_char == '=') {
                        punct_len = 2;
                    }
                }
                let str1 = &arg[start..=i+punct_len-1];
                let token = Token {
                    kind: TokenKind::TkPunct,
                    value: Some(V::Str(str1)),
                    loc: i,
                    len: punct_len,
                };
                tokens.push(token);
                start = i + punct_len;
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
                // i = start;
                // continue;
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
    NdNeg, // 负号-
    NdEq,  // ==
    NdNeq,  // !=
    NdLt,  // <
    NdLe,  // <=
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
    fn new_unary(kind:NodeKind, single_node: Box<Node>) -> Box<Node> {
        let mut node = Node::new_node(kind);
        node.lhs = Some(single_node);
        node
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
    let expression = expr(tokens, &mut pos)?;
    if pos != tokens.len() - 1 {
        return Err(String::from("Unexpected tokens"));
        // error!("Unexpected tokens")
    }
    Ok(expression)
}
/*====================================================================
                        expr - 生成AST的实际顶层
                            解析表达式
                          expr = equality
 ====================================================================*/
 fn expr(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    equality(tokens,pos)
}


/*====================================================================
                        equality - 解析!= == 
        equality = relational ("==" relational | "!=" relational)*
 ====================================================================*/
fn equality(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    // relational
    let mut node = relational(tokens, pos)?;

    // ("==" relational | "!=" relational)*
    loop {
        let tmp = tokens.get(*pos);
        match tmp {
            tmp if equal(tmp.unwrap(), "==") => {
                *pos += 1;
                let right = relational(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdEq, node, right);
            }
            tmp if equal(tmp.unwrap(), "!=") => {
                *pos += 1;
                let right = relational(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdNeq, node, right);
            }
            _ => { break; }
        }
    }

    Ok(node)
}

/*====================================================================
                        relational - 解析比较关系
        relational = add ("<" add | "<=" add | ">" add | ">=" add)*
 ====================================================================*/
fn relational(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    let mut node = add(tokens, pos)?;

    loop {
        let tmp = tokens.get(*pos);
        match tmp {
            tmp if equal(tmp.unwrap(), "<") => {
                *pos += 1;
                let right = add(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdLt, node, right);
            }
            tmp if equal(tmp.unwrap(), "<=") => {
                *pos += 1;
                let right = add(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdLe, node, right);
            }
            tmp if equal(tmp.unwrap(), ">") => {
                *pos += 1;
                let right = add(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdLt, right, node);
            }
            tmp if equal(tmp.unwrap(), ">=") => {
                *pos += 1;
                let right = add(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdLe, right, node);
            }
            _ => break,
        }
    }

    Ok(node)
}

/*====================================================================
                            add - 解析加减
                    add = mul ("+" mul | "-" mul)*
 ====================================================================*/
// add函数的实现
fn add(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    let mut node = mul(tokens, pos)?;

    loop {
        let tmp = tokens.get(*pos);

        match tmp {
            Some(Token{kind: TokenKind::TkPunct, value: Some(V::Str("+")), ..}) => {
                *pos += 1;
                let right = mul(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdAdd, node, right);
            },
            Some(Token{kind: TokenKind::TkPunct, value: Some(V::Str("-")), ..}) => {
                *pos += 1;
                let right = mul(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdSub,node,right);
            },
            _ => break,
        }
    }

    Ok(node)
}
/*====================================================================
                        mul - 乘数节点生成模块
                mul = unary ("*" unary | "/" unary)*
                    乘数本身是由一元运算数相乘或相除得到的
 ====================================================================*/
 fn mul(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    let mut node = unary(tokens, pos)?;
    // let mut node = primary(tokens, pos)?;
    loop {
        let tmp = tokens.get(*pos); 
        match tmp {
            tmp if equal(tmp.unwrap(),"*")=> {
                *pos += 1;
                let right = unary(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdMul, node, right);
            }
            tmp if equal(tmp.unwrap(),"/") => {
                *pos += 1;
                let right = unary(tokens, pos)?;
                node = Node::new_binary(NodeKind::NdDiv, node, right);
            }
            _ => { break; }
        }
    }
    Ok(node)
}
/*====================================================================
                        unary - 一元运算数节点生成
                      unary = ("+" | "-") unary | primary
                        unary可以是±，也可以直接是一个primary
 ====================================================================*/
fn unary(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
     if let Some(tok) = tokens.get(*pos) {
        if equal(tok, "+") {
            *pos += 1;
            let node = unary(tokens, pos)?;
            return Ok(node);
        }

        // "-" unary
        if equal(tok, "-") {
            *pos += 1;
            let node = unary(tokens,pos)?;
            let node = Node::new_unary(NodeKind::NdNeg, node);
            return Ok(node);
        }
    }


    // primary
    primary(tokens,pos)
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
            let tok = tmp.unwrap();
            error!(tok, "expected an expression");
        }
    }
    
}


/*====================================================================
                        代码生成模块 
                         栈管理函数
 ====================================================================*/

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

/*======================================================================
                        GenAsm: 代码生成主干函数
                接收目标字符串，接入Token处理为RISC-V汇编
====================================================================== */
fn gen_asm(nd:Box<Node> ){

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
    // 增加一层检查：如果栈未清空则报错
    unsafe{
        assert!(DEPTH==0);
    }
}