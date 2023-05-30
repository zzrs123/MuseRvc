/*======================================================================
                    AST 抽象语法树 系统 == 语法解析
 =======================================================================*/
 // 本地变量

use std::vec;

use crate::lexer::Token;
use crate::typecheck::*;
use crate::error;
use crate::error::verror_at;   
pub struct  Program {
    pub funclist: Vec<Function>,
}
impl Program{
    pub fn new()-> Self{
        Self { funclist: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Obj {
    //  pub(crate) next: Option<Box<Obj>>,  // 指向下一对象
     pub name: String,            // 变量名
     pub offset: i32,             // fp的偏移量
     pub ty: Option<Type>, //变量类型（step10预留） step20的时候考虑启用，又因为后续用不到而放弃。
     // is_local: bool 是否为本地变量（step10预留）
 }
impl Obj {
    pub fn new(name: String, offset: i32, ty:&Type) -> Self {
        Obj {
            // next: None,
            name,
            offset,
            ty: Some(ty.clone()),
        }
    }
}
 // 函数，目前函数只有变量+表达式

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    // pub next: Option<Box<Function>>, // 下一函数，这里其实想用列表
    pub name: String,       // 函数名
    pub body: Option<Box<Node>>,  // 函数体
    pub locals: Vec<Obj>, // 本地变量
    pub stack_size: i32,          // 栈大小
 }
impl Function {
     pub fn new() -> Self {
        Self {
        //    name: name.to_string(),
            name: "".to_string(),
            body: None,
            locals:vec![],
            stack_size:0,
            // next: None,
        }
     }
     pub fn find_local_var(&self, name: &str) -> Option<&Obj> {
        for var in self.locals.iter() {
            if var.name == name {
                return Some(var);
            }
        }
        None
    }

    pub fn add_local_var(&mut self, name: String, ty: &Type) -> Option<Obj> {
        // let index = self.locals.len();
        // let offset = func.stack_size;
        let obj = Obj::new(name, 0, ty);
        self.locals.push(obj.clone());   
        
        Some(obj)
    }
     
 }

 // // 解析函数定义  (一个设想，留待后面参考)
     // fn function(tokens: &Vec<Token>, pos: &mut usize) -> Result<Box<Node>, String> {
     //      // 解析函数名
     //      let name = expect_ident(tokens, pos)?;
     
     //      // 创建函数对象和locals链表
     //      let mut func = Function::new(&name);
     //      let locals = Box::new(Obj::new_block(0));
     //      func.locals = Some(locals);
     
     //      // 解析函数参数列表
     //      let params = params(tokens, pos)?;
     
     //      // 解析函数体
     //      let body = stmt_block(tokens, pos)?;
     //      func.body = Some(body);
     
     //      // 将函数对象加入到全局变量中
     //      add_function(func);
     
     //      // 返回空语句节点
     //      Ok(Node::new_empty())
     // }
 #[derive(Debug, PartialEq, Clone)]
 // AST的节点种类
pub enum NodeKind {
     NdAdd,       // +
     NdSub,       // -
     NdMul,       // *
     NdDiv,       // /
     NdNeg,       // 负号-
     NdEq,        // ==
     NdNeq,       // !=
     NdLt,        // <
     NdLe,        // <=
     NdNum,       // 整型数字
     NdAssign,    // 赋值=
     NdAddr,      // 取地址 &
     NdDeref,     // 解引用 *
     NdReturn,    // 返回
     NdIf,        // "if"，条件判断
     NdFor,       // "for" 或 "while"，循环
     NdEmpty,     // 空语句
     NdBlock,     // { ... }，代码块
     NdFuncall,   // 函数调用
     NdExprStmt,  // 表达式
     NdVar,       // 变量
 }
 // AST中二叉树节点
 #[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub kind: NodeKind, // 节点种类
    pub next: Option<Box<Node>>, // 下一节点，指代下一语句  !!!
    pub lhs: Option<Box<Node>>, // 左部，left-hand side
    pub rhs: Option<Box<Node>>, // 右部，right-hand side

    pub tokloc: usize,          // tok's loc
    pub ty:  Option<Type>,      // Node's type{int or ptr}
    // "if"语句 或者 "for"语句
    pub cond: Option<Box<Node>>, // 条件内的表达式
    pub then: Option<Box<Node>>, // 符合条件后的语句
    pub els : Option<Box<Node>>, // 不符合条件后的语句
    pub init: Option<Box<Node>>, // 初始化语句
    pub inc : Option<Box<Node>>,  // 递增语句

    pub body: Option<Box<Node>>, // 代码块 !!!

    pub val: Option<i32>, // 存储ND_NUM种类的值，不需要存储操作符（类型判断）
    pub name: Option<String>, // 存储ND_VAR的字符串
    pub var: Option<Obj>,    // 存储ND_VAR种类的变量

    pub funcname: String,   // 函数名
    pub args: Option<Box<Node>>, // 参数列表
    
    
}
 impl Node {
     pub fn new_node(kind: NodeKind) -> Box<Node> {
         Box::new(Node {
            kind,
            next: None,
            lhs: None,
            rhs: None,
            tokloc: 0,
            ty: None, //本来打算裸Type，考虑这里还是使用Option了
            body:None,
            var: None,
            val: None,
            name: None,
            cond: None,
            then:None,
            els: None,
            init: None,
            inc:  None,
            funcname: "".to_owned(),
            args: None,
        })
     }

    pub fn new_unary(kind:NodeKind, single_node: Box<Node>, tok: &Token) -> Box<Node> {
         let mut node = Node::new_node(kind);
         node.lhs = Some(single_node);
         node.tokloc = tok.loc;
         node
    }

    pub fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>, tok: &Token) -> Box<Node> {
         let mut node = Node::new_node(kind);
         node.lhs = Some(lhs);
         node.rhs = Some(rhs);
         node.tokloc = tok.loc;
         node
    }
 
    pub fn new_num(val: i32, tok: &Token) -> Box<Node> {
         let mut node = Node::new_node(NodeKind::NdNum);
         node.val = Some(val);
         node.name = Some(val.to_string());
         node.tokloc = tok.loc;
         node
    }

    pub fn new_var(var:Obj, tok: &Token)-> Box<Node> {
          let mut node = Node::new_node(NodeKind::NdVar);
          node.name = Some(var.clone().name);
          node.var = Some(var);
          node.tokloc = tok.loc;
          node
     }
    //  new_add 和 new_sub 其实跟上面几个new不太一样，是对new_binary等的上位封装
    // 这个封装考虑了类型系统
    pub fn new_add(mut lhs: Box<Node>, mut rhs: Box<Node>, tok: &Token) -> Box<Node> {
         
        // 为左右部添加类型
        let mut lhs = Type::add_type(&mut lhs);
        let mut rhs = Type::add_type(&mut rhs);
        match (lhs.clone().ty, rhs.clone().ty) {
            (Some(lhs_ty), Some(rhs_ty)) => {
                // 检查左右子节点的类型是否都为整数类型
                if Type::is_integer(&lhs_ty) && Type::is_integer(&rhs_ty) {
                    // 如果左右子节点均为整数节点，创建加法节点
                    return Node::new_binary(NodeKind::NdAdd, lhs, rhs, tok);
                }
                // 不能解析 ptr + ptr
                if lhs_ty.clone().base_ty().is_some() && rhs_ty.clone().base_ty().is_some() {
                    error!(tok, "invalid operands ");
                }

                // 将 num + ptr 转换为 ptr + num
                if lhs_ty.clone().base_ty().is_none() && rhs_ty.clone().base_ty().is_some() {
                    let tmp = lhs;
                    lhs = rhs;
                    rhs = tmp;
                }

                // ptr + num
                // 指针加法，ptr+1，这里的1不是1个字节，而是1个元素的空间，所以需要 ×8 操作
                rhs = Node::new_binary(NodeKind::NdMul, rhs, Node::new_num(8, tok), tok);
                return Node::new_binary(NodeKind::NdAdd, lhs, rhs, tok);
            }
            // 没想好怎么处理
            (None, None) => todo!(),
            (None, Some(_)) => todo!(),
            (Some(_), None) => todo!(),

        }
    }
    
    pub fn new_sub(mut lhs: Box<Node>,mut rhs: Box<Node>, tok: &Token) -> Box<Node> {
        // 为左右部添加类型
        let lhs = Type::add_type(&mut lhs);
        let rhs = Type::add_type(&mut rhs);

        match (lhs.ty.clone(), rhs.ty.clone()) {
            (Some(lhs_ty), Some(rhs_ty)) => {
                // num - num
                if Type::is_integer(&lhs_ty) && Type::is_integer(&rhs_ty) {
                    return Node::new_binary(NodeKind::NdSub, lhs, rhs, tok);
                }
                // ptr - num
                if Type::is_pointer(&lhs_ty) && Type::is_integer(&rhs_ty) {
                    let mut rhs = Node::new_binary(NodeKind::NdMul, rhs, Node::new_num(8, tok), tok);
                    Type::add_type(&mut rhs);
                    let mut nd = Node::new_binary(NodeKind::NdSub, lhs.clone(), rhs, tok);
                    nd.ty = lhs.ty.clone();
                    return nd;
                } 
                // ptr - ptr，返回两指针间有多少元素
                if lhs_ty.base_ty().is_some() && rhs_ty.base_ty().is_some() {
                    let mut nd = Node::new_binary(NodeKind::NdSub, lhs, rhs, tok);
                    nd.ty = Some(Type::new(TypeKind::Int));
                    return Node::new_binary(NodeKind::NdDiv, nd, Node::new_num(8, tok), tok);
                } else {
                    error!(tok, "invalid operands");
                }
            },
            (None, None) => todo!(),
            (None, Some(_)) => todo!(),
            (Some(_), None) => todo!(),
        }
    }
    
}


