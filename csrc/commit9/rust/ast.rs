/*======================================================================
                    AST 抽象语法树 系统 == 语法解析
 =======================================================================*/
 
 #[derive(Debug, PartialEq, Clone)]
 // AST的节点种类
pub enum NodeKind {
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
     NdExprStmt,// 表达式
     NdEmpty,//空语句
 }
 // AST中二叉树节点
 #[derive(Debug, PartialEq, Clone)]
pub struct Node {
     pub kind: NodeKind, // 节点种类
     pub next: Option<Box<Node>>, // 下一节点，指代下一语句
     pub lhs: Option<Box<Node>>, // 左部，left-hand side
     pub rhs: Option<Box<Node>>, // 右部，right-hand side
     pub val: Option<i32>, // 存储ND_NUM种类的值，不需要存储操作符（类型判断）
 }
 impl Node {
    pub fn new_node(kind: NodeKind) -> Box<Node> {
         Box::new(Node {
             kind,
             next:None,
             lhs: None,
             rhs: None,
             val: None,
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
 }