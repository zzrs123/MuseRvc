use crate::error;
use crate::error::verror_at;

/*=====================================================================
                    Token系统
// ====================================================================== */
#[derive(PartialEq)]
pub enum TokenKind {
    TkPunct, // 操作符如： + -
    TkNum,   // 数字
    TKEof,   // 文件终止符，即文件的最后
}

pub enum V<'a> {
    Int(i32),
    Str(&'a str),
}
#[allow(dead_code)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub value: Option<V<'a>>,
    pub loc: usize,
    pub len: usize,
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
pub fn tokenize(arg: &mut str) -> Vec<Token> {
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
