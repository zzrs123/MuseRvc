/*======================================================================
                    AST 抽象语法树 系统 == 语法解析
 =======================================================================*/
 // 本地变量


#[derive(Debug, PartialEq, Clone)]
pub struct Obj {
    //  pub(crate) next: Option<Box<Obj>>,  // 指向下一对象
     pub name: String,            // 变量名
     pub offset: i32,             // fp的偏移量
     // type: Type; 变量类型（step10预留）
     // is_local: bool 是否为本地变量（step10预留）
 }
impl Obj {
    pub fn new(name: String, offset: i32) -> Self {
        Obj {
            // next: None,
            name,
            offset,
        }
    }
}
 // 函数，目前函数只有变量+表达式

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
     // 函数名
     // pub name: String, // 函数名（step10预留）
     pub body: Option<Box<Node>>,  // 函数体
     pub locals: Vec<Obj>, // 本地变量
     pub stack_size: i32,          // 栈大小
 }
impl Function {
     pub fn new() -> Self {
        Self {
        //    name: name.to_string(),
            body: None,
            locals:vec![],
            stack_size:0,
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

    pub fn add_local_var(&mut self, name: String) -> Option<Obj> {
        // let index = self.locals.len();
        // let offset = func.stack_size;
        let obj = Obj::new(name, 0);
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
     NdReturn,    // 返回
     NdIf,        // "if"，条件判断
     NdFor,       // "for" 或 "while"，循环
     NdEmpty,     //空语句
     NdBlock,     // { ... }，代码块
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
 }
 impl Node {
     pub fn new_node(kind: NodeKind) -> Box<Node> {
         Box::new(Node {
            kind,
            next: None,
            lhs: None,
            rhs: None,
            body:None,
            var: None,
            val: None,
            name: None,
            cond: None,
            then:None,
            els: None,
            init: None,
            inc:  None,
        })
     }

    pub fn new_unary(kind:NodeKind, single_node: Box<Node>) -> Box<Node> {
         let mut node = Node::new_node(kind);
         node.lhs = Some(single_node);
         node
    }

    pub fn new_binary(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
         let mut node = Node::new_node(kind);
         node.lhs = Some(lhs);
         node.rhs = Some(rhs);
         node
    }
 
    pub fn new_num(val: i32) -> Box<Node> {
         let mut node = Node::new_node(NodeKind::NdNum);
         node.val = Some(val);
         node
    }

    pub fn new_var(var:Obj)-> Box<Node> {
          let mut node = Node::new_node(NodeKind::NdVar);
          node.name = Some(var.clone().name);
          node.var = Some(var);
          node
     }

}