/*=====================================================================
                        词法分析
// ====================================================================== */

// use crate::parser::equal;
use crate::{error};
use crate::error::verror_at;
// use crate::parser::equal;
/*=====================================================================
                        Token系统
// ====================================================================== */
#[derive(PartialEq, Clone)]
pub enum TokenKind {
    TkIdent, // 标记符，可以为变量名、函数名等
    TkPunct, // 操作符如： + -
    TkKeyword, // 关键字
    TkNum,   // 数字
    TKEof,   // 文件终止符，即文件的最后
}

pub enum V<'a> {
    Int(i32),
    Str(&'a str), //step9：2023/05/05 感觉这里用&str不优雅，后续看看能不能用enum嵌套或者String
}
#[allow(dead_code)]
// 终结符结构体
pub struct Token<'a> {
    pub kind: TokenKind,
    pub value: Option<V<'a>>,
    pub loc: usize,
    pub len: usize,
}

/*======================================================================
                    is_keywords: 关键词判断函数
                维护一个关键词表，传入token返回true false
====================================================================== */
fn is_keywords(token: &Token) -> bool{
    let keys = vec!["return", "if", "else"];
    if let Some(V::Str(st)) = token.value{
        if keys.contains(&st){
            return true;
        }else{
            return false
        }
    }
    false
}

/*======================================================================
                    convert_keywords: 关键词识别函数
                将tokens序列中的关键词识别出来转化为keword类型
====================================================================== */
fn convert_keywords(tokens:&mut Vec<Token>){
    for tok in tokens.iter_mut() {
        // if parser::equal(&tok, "return") {
        //     tok.kind = TokenKind::TkKeyword;
        // }
        if is_keywords(&tok){
            tok.kind = TokenKind::TkKeyword;
        }
    }
}

/*======================================================================
                    toknize: Token解析主干函数
            从头到尾扫描args[1]，对不同类型的token做不同处理
                rust用match+vec<T>实现起来是相当优雅的

                step9：2023/05/05：太长了这个函数
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
                };
                tokens.push(token);
                start = end;
            }
            // 解析字母（变量），同时预留了多字母变量的支持
            'a'..='z' | 'A'..='Z' | '_'  => {
                let mut end = i;
                while let Some(c) = arg.chars().nth(end) {
                    if c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_' {
                        end += 1;
                    } else {
                        break;
                    }
                }
                let ident = &arg[start..end];
                let token = Token {
                    kind: TokenKind::TkIdent,
                    value: Some(V::Str(ident)),
                    loc: i,
                    len: end - start,
                };
                tokens.push(token);
                start = end;
            }
            _ => {
                // 处理无法识别的字符
                let loc_char: usize = i;
                error!(loc_char,"Unexpected character '{}'", c);
            }
        }
    }
    let eof_token = Token {
        kind: TokenKind::TKEof,
        value: None,
        loc: arg.len(),
        len: 0,
    };
    tokens.push(eof_token);
    convert_keywords(&mut tokens);
    tokens
}
