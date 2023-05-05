use crate::lexer::{V, TokenKind,Token};
use crate::ast::{Node,NodeKind};
use crate::error;
use crate::error::verror_at;    
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

/*====================================================================
                        parser - 生成AST的逻辑顶层
                    主要负责实现 Token vec 与 二叉树 的连接
 ====================================================================*/
 pub fn parser(tokens: &Vec<Token>) -> Result<Box<Node>, String> {
    let mut pos = 0;
    let mut head = Node::new_node(NodeKind::NdEmpty);
    let mut cur = &mut head;

    while pos < tokens.len()-1 {
        let node = stmt(tokens, &mut pos)?;
        cur.next = Some(node);
        cur = cur.next.as_mut().unwrap();
    }

    Ok(head.next.unwrap())
}
/*====================================================================
                            stmt - 程序语句
                目前 stmt == expr_stmt，也就是只有常数表达式
 ====================================================================*/
fn stmt(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    expr_stmt(tokens, pos)
}

// 解析表达式语句
// exprStmt = expr ";"
fn expr_stmt(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
    let node = expr(tokens, pos)?;
    let tmp = tokens.get(*pos).unwrap();
    match tmp {
        tmp if equal(tmp, ";")=> {
            *pos += 1;
            let node = Node::new_unary(NodeKind::NdExprStmt, node);
            Ok(node)
        }
        _ => Err(String::from("Expected ';' after expression")),
    }
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