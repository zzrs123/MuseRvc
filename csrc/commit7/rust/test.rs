
// 解析表达式
// expr = equality
fn expr(rest: &mut Option<Token>) -> Option<Box<Node>> {
    equality(rest)
}
// / 解析相等性
// equality = relational ("==" relational | “!=” relational)*
fn equality(rest: &mut Token, mut tok: Token) -> Node {
// relational
let mut nd = relational(&mut tok, tok);

// ("==" relational | "!=" relational)*
loop {
    // "==" relational
    if equal(&tok, "==") {
        nd = newBinary(ND_EQ, nd, relational(&mut tok, tok.next.clone().unwrap()));
        continue;
    }

    // "!=" relational
    if equal(&tok, "!=") {
        nd = newBinary(ND_NE, nd, relational(&mut tok, tok.next.clone().unwrap()));
        continue;
    }

    *rest = tok;
    return nd;
}

}

// 解析比较关系
// relational = add ("<" add | “<=” add | “>” add | “>=” add)*
fn relational(rest: &mut Token, mut tok: Token) -> Node {
// add
let mut nd = add(&mut tok, tok);

// ("<" add | "<=" add | ">" add | ">=" add)*
loop {
    // "<" add
    if equal(&tok, "<") {
        nd = newBinary(ND_LT, nd, add(&mut tok, tok.next.clone().unwrap()));
        continue;
    }

    // "<=" add
    if equal(&tok, "<=") {
        nd = newBinary(ND_LE, nd, add(&mut tok, tok.next.clone().unwrap()));
        continue;
    }

    // ">" add
    // X>Y等价于Y<X
    if equal(&tok, ">") {
        nd = newBinary(ND_LT, add(&mut tok, tok.next.clone().unwrap()), nd);
        continue;
    }

    // ">=" add
    // X>=Y等价于Y<=X
    if equal(&tok, ">=") {
        nd = newBinary(ND_LE, add(&mut tok, tok.next.clone().unwrap()), nd);
        continue;
    }

    *rest = tok;
    return nd;
}

}

// 解析加减
// add = mul ("+" mul | “-” mul)*
fn add(rest: &mut Token, mut tok: Token) -> Node {
// mul
let mut nd = mul(&mut tok, tok);

// ("+" mul | "-" mul)*
loop {
    // "+" mul
    if equal(&tok, "+") {
        nd = newBinary(ND_ADD, nd, mul(&mut tok, tok.next.clone().unwrap()));
        continue;
    }

    // "-" mul
    if equal(&tok, "-") {
        nd = newBinary(ND_SUB, nd, mul(&mut tok, tok.next.clone().unwrap()));
        continue;
    }

    *rest = tok;
    return nd;
}

}