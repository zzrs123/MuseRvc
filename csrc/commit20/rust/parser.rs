/*======================================================================
                        文法/语法 分析
====================================================================== */

use crate::lexer::{V, TokenKind,Token};
use crate::ast::{Node,NodeKind,Function};
use crate::error;
use crate::error::verror_at;   

/*======================================================================
                    equal: 字符类型/操作符匹配函数
                比较Token的字符value与传入的参数字符串是否相等
                            返回布尔值
====================================================================== */
pub fn equal(token: &Token, s: &str) -> bool {

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
                    主要负责实现 Token vec 与 语法树 的连接
                        program = "{" compoundStmt
 ====================================================================*/
 pub fn parser(tokens: &Vec<Token>) -> Result<Function, String> {
    let mut pos = 0;
    let mut func = Function::new();
    // 这里实际上是跳过"{"
    // pos += 1;  所以改写为下面的方式
    match equal(&tokens[pos],"{") {
        true => {
            pos+=1; // 跳过去
        }
        false => error!("expect a {{"),
    }
    // step12: 其实我还考虑将下面的函数s的func参数改为locals参数，不用每次传入func，只需locals就可以
    // 返回到parser中时再用  `func.locals = locals;` 即可
    func.body = Some(compound_stmt(tokens, &mut pos, &mut func)?);
    Ok(func)
}

/*====================================================================
                        compound_stmt - 解析代码块
                        compoundStmt = stmt* "}"
 ====================================================================*/
fn compound_stmt(tokens: &Vec<Token>, pos: &mut usize, func: &mut Function) -> Result<Box<Node>, String> {
    let mut head = Node::new_node(NodeKind::NdEmpty);
    let mut cur = &mut head;
    // 这里原本是parser做的事情，下放到compound_stmt
    while !equal(&tokens[*pos], "}") && *pos < tokens.len()-1 {
        let node = stmt(tokens, pos, func)?;
        cur.next = Some(node);
        cur = cur.next.as_mut().unwrap();
    }
    // 增加错误判断，使得尾括号缺失时报错
    if !equal(&tokens[*pos], "}") {
        error!("Expected '}}' after compound statement")
    }
    // Nd的Body存储了{}内解析的语句
    *pos += 1;
    let mut nd = Node::new_node(NodeKind::NdBlock);
    nd.body = head.next;
    Ok(nd)
}

/*====================================================================
                            stmt - 程序语句
                    stmt = "return" expr? ";"
                            | "if" "(" expr ")" stmt ("else" stmt)?
                            | "for" "(" exprStmt expr? ";" expr? ")" stmt
                            | "while" "(" expr ")" stmt
                            | "{" compoundStmt
                            | exprStmt
 ====================================================================*/
fn stmt(tokens: &Vec<Token>, pos: &mut usize, func: &mut Function) -> Result<Box<Node>, String> {
    let tmp = tokens.get(*pos).unwrap();
    // let tmp = tokens.get(*pos).unwrap();
    if equal(tmp, ";") {
        *pos += 1;
        let node = Node::new_node(NodeKind::NdBlock);
        return Ok(node);
    }
    match tmp.kind {
        TokenKind::TkKeyword => {
            *pos += 1;
            if equal(tmp.clone(),"return"){
                let expr_node = expr(tokens, pos, func)?;
                if !equal(&tokens[*pos], ";") {
                    error!(tmp,"Expected ';' after return expression");
                }
                *pos += 1;
                let node = Node::new_unary(NodeKind::NdReturn, expr_node, tmp.clone());
                return Ok(node)
            }
            // 解析if语句
            // "if" "(" expr ")" stmt ("else" stmt)?
            else if equal(tmp.clone(), "if"){
                let mut node = Node::new_node(NodeKind::NdIf);
                // *pos += 1;
                let tmp1 = &tokens[*pos];
                if !equal(tmp1, "(") {
                    error!(tmp1,"Expected '(' after if");
                }
                *pos += 1;
                node.cond = Some(expr(tokens, pos, func)?);
                if !equal(&tokens[*pos], ")") {
                    error!(tmp,"Expected ')' after if condition");
                }
                *pos += 1;
                node.then = Some(stmt(tokens, pos, func)?);
                if equal(&tokens[*pos], "else") {
                    *pos += 1;
                    node.els = Some(stmt(tokens, pos, func)?);
                }
                return Ok(node);
            } 
            // "for" "(" exprStmt expr? ";" expr? ")" stmt
            else if equal(tmp, "for") {
                // *pos += 1;
                let mut nd = Node::new_node(NodeKind::NdFor);
                // *pos += 1;
                if !equal(&tokens[*pos], "(") {
                    error!(tmp,"Expected '(' after for");
                }
                *pos += 1;
                // 注意这里调用stmt，因为可能存在for(;;)的情况，而空语句我的设定是在stmt中处理的
                nd.init = Some(stmt(tokens, pos, func)?);
                if !equal(&tokens[*pos], ";") {
                    nd.cond = Some(expr(tokens, pos, func)?);
                }
                *pos += 1;
                if !equal(&tokens[*pos], ")") {
                    nd.inc = Some(expr(tokens, pos, func)?);
                }
                *pos += 1;
                nd.then = Some(stmt(tokens, pos, func)?);
                // *rest = &tokens[*pos..];
                return Ok(nd);
            }
            // “while” “(” expr “)” stmt
            if equal(tmp, "while") {
                let mut nd = Node::new_node(NodeKind::NdFor);
                // “(”
                if !equal(&tokens[*pos], "(") {
                error!(tmp,"Expected '(' after while");
                }
                *pos += 1;
                // expr
                nd.cond = Some(expr(tokens, pos, func)?);
                // “)”
                if !equal(&tokens[*pos], ")") {
                error!(tmp,"Expected ')' after condition in while statement");
                }
                *pos += 1;
                // stmt
                nd.then = Some(stmt(tokens, pos, func)?);
                Ok(nd)
            }
            
            else {
                error!("nod")
            }  
        }
        // step19：这里原先的else是直接error!报错遇到了不可识别的token，
        // 但实际上这样就无法识别*y=5;这样的语句中的*号
        TokenKind::TkPunct => {
            if equal(tmp.clone(), "{") {
                *pos += 1;
                compound_stmt(tokens, pos, func)
            } else {
                expr_stmt(tokens, pos, func)
            }
        }
        _ => expr_stmt(tokens, pos, func),
    }
}

/*====================================================================
                         expr_stmt- 解析表达式语句
                        exprStmt = expr ";"
        空语句我放在了stmt中来解析，如果放在expr_stmt中，则文法是expr? ";"
 ====================================================================*/
fn expr_stmt(tokens: &Vec<Token>, pos: &mut usize, func: &mut Function) -> Result<Box<Node>, String> {

    let node = expr(tokens, pos, func)?;
    let tmp = tokens.get(*pos).unwrap();
    match tmp {
        tmp if equal(tmp, ";")=> {
            *pos += 1;
            let node = Node::new_unary(NodeKind::NdExprStmt, node,tmp.clone());
            Ok(node)
        }
        _ => Err(String::from("Expected ';' after expression")),
    }
}
/*====================================================================
                        expr - 解析表达式
                        expr = assign
 ====================================================================*/
 fn expr(tokens: &Vec<Token>, pos: &mut usize,func: &mut Function) -> Result<Box<Node>, String> {
    assign(tokens, pos, func)
}

/*====================================================================
                       assign - 解析赋值
                assign = equality ("=" assign)?
 ====================================================================*/
fn assign(tokens: &Vec<Token>, pos: &mut usize,func: &mut Function) -> Result<Box<Node>, String> {
    let mut node = equality(tokens, pos, func)?;
    // 可能存在递归赋值，如a=b=1
    // ("=" assign)?
    if let Some(tmp) = tokens.get(*pos) {
        if equal(tmp, "=") {
            *pos += 1;
            let rhs = assign(tokens, pos, func)?;
            node = Node::new_binary(NodeKind::NdAssign, node, rhs, tmp.clone());
        }
    }
    Ok(node)
}

/*====================================================================
                        equality - 解析相等性
        equality = relational ("==" relational | "!=" relational)*
 ====================================================================*/
fn equality(tokens: &Vec<Token>, pos: &mut usize,func: &mut Function) -> Result<Box<Node>, String> {
    // relational
    let mut node = relational(tokens, pos, func)?;

    // ("==" relational | "!=" relational)*
    loop {
        let tmp = tokens.get(*pos);
        match tmp {
            tmp if equal(tmp.unwrap(), "==") => {
                *pos += 1;
                let right = relational(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdEq, node, right, tmp.clone().unwrap());
            }
            tmp if equal(tmp.unwrap(), "!=") => {
                *pos += 1;
                let right = relational(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdNeq, node, right, tmp.clone().unwrap());
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
fn relational(tokens: &Vec<Token>, pos: &mut usize,func: &mut Function) -> Result<Box<Node>, String> {
    let mut node = add(tokens, pos, func)?;

    loop {
        let tmp = tokens.get(*pos);
        match tmp {
            tmp if equal(tmp.unwrap(), "<") => {
                *pos += 1;
                let right = add(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdLt, node, right, tmp.clone().unwrap());
            }
            tmp if equal(tmp.unwrap(), "<=") => {
                *pos += 1;
                let right = add(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdLe, node, right, tmp.clone().unwrap());
            }
            tmp if equal(tmp.unwrap(), ">") => {
                *pos += 1;
                let right = add(tokens, pos,func)?;
                node = Node::new_binary(NodeKind::NdLt, right, node, tmp.clone().unwrap());
            }
            tmp if equal(tmp.unwrap(), ">=") => {
                *pos += 1;
                let right = add(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdLe, right, node, tmp.clone().unwrap());
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
fn add(tokens: &Vec<Token>, pos: &mut usize,func: &mut Function) -> Result<Box<Node>, String> {
    let mut node = mul(tokens, pos, func)?;

    loop {
        let tmp = tokens.get(*pos);

        match tmp {
            Some(Token{kind: TokenKind::TkPunct, value: Some(V::Str("+")), ..}) => {
                *pos += 1;
                let right = mul(tokens, pos,func)?;
                node = Node::new_binary(NodeKind::NdAdd, node, right, tmp.clone().unwrap());
            },
            Some(Token{kind: TokenKind::TkPunct, value: Some(V::Str("-")), ..}) => {
                *pos += 1;
                let right = mul(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdSub,node,right, tmp.clone().unwrap());
            },
            _ => break,
        }
    }

    Ok(node)
}
/*====================================================================
                            mul - 解析乘除
                mul = unary ("*" unary | "/" unary)*
                   乘数本身是由一元运算数相乘或相除得到的
 ====================================================================*/
 fn mul(tokens: &Vec<Token>, pos: &mut usize,func: &mut Function) -> Result<Box<Node>, String> {
    let mut node = unary(tokens, pos, func)?;
    // let mut node = primary(tokens, pos)?;
    loop {
        let tmp = tokens.get(*pos); 
        match tmp {
            tmp if equal(tmp.unwrap(),"*")=> {
                *pos += 1;
                let right = unary(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdMul, node, right, tmp.clone().unwrap());
            }
            tmp if equal(tmp.unwrap(),"/") => {
                *pos += 1;
                let right = unary(tokens, pos, func)?;
                node = Node::new_binary(NodeKind::NdDiv, node, right, tmp.clone().unwrap());
            }
            _ => { break; }
        }
    }
    Ok(node)
}
/*====================================================================
                            unary - 解析一元运算
                unary = ("+" | "-" | "*" | "&") unary | primary
                    unary可以是±，也可以直接是一个primary
 ====================================================================*/
fn unary(tokens: &Vec<Token>, pos: &mut usize,func: &mut Function) -> Result<Box<Node>, String> {
     
    if let Some(tok) = tokens.get(*pos) {
        // "+" unary
        if equal(tok, "+") {
            *pos += 1;
            let node = unary(tokens, pos, func)?;
            return Ok(node);
        }

        // "-" unary
        if equal(tok, "-") {
            *pos += 1;
            let node = unary(tokens,pos, func)?;
            let node = Node::new_unary(NodeKind::NdNeg, node, tok.clone());
            return Ok(node);
        }

        // "&" unary
        if equal(tok, "&") {
            *pos += 1;
            let node = unary(tokens,pos, func)?;
            let node = Node::new_unary(NodeKind::NdAddr, node, tok.clone());
            return Ok(node);
        }

        // "*" unary
        if equal(tok, "*") {
            *pos += 1;
            let node = unary(tokens,pos, func)?;
            let node = Node::new_unary(NodeKind::NdDeref, node, tok.clone());
            return Ok(node);
        }
    }

    // primary
    primary(tokens,pos, func)
}

/*====================================================================
                        primary - 解析括号、数字、变量
                    primary = "(" expr ")" | ident｜ num:
 ====================================================================*/
 fn primary(tokens: &Vec<Token>, pos: &mut usize, func:&mut Function) -> Result<Box<Node>, String> {
    // "(" expr ")"
    let tmp = tokens.get(*pos);
    match tmp {
        tmp if equal(tmp.unwrap(), "(") => {
            *pos += 1;
            let node = expr(tokens, pos, func)?;
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
            let node = Node::new_num(nnn, tmp.clone().unwrap());
            Ok(node)
        }
        tmp if tmp.unwrap().kind == TokenKind::TkIdent => {
            *pos += 1;
            let var_name;
            if let Some(V::Str(s)) = tmp.unwrap().value {
                var_name = s.to_string();
            } else {
                return Err("expect variable name".to_string());
            }
            if let Some(var) = func.find_local_var(&var_name) {
                // 如果 locals 中已经存在该变量，则创建一个 NdLocalVar 节点，并返回
                // let offset = func.locals[index].offset;
                let node = Node::new_var(var.clone(), tmp.clone().unwrap());
                Ok(node)
            } else {
                let obj = func.add_local_var(var_name);
                let node = Node::new_var(obj.unwrap().clone(), tmp.clone().unwrap());
                Ok(node)
            }
        
        }

        _ => {
            let tok = tmp.unwrap();
            error!(tok, "expected an expression");
        }
    }
}
