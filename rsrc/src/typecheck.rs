use crate::ast::*;
/*======================================================================
                            Type 类型推理系统
                    后续可能会成为一个类型推理、检测系统
 =======================================================================*/
 
// 类型种类
#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    // Unchecked, //未定义类型，step20预留，我觉得一定会有这个类型
    Int, // 整型
    Ptr, // 指针
}
#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    pub kind: TypeKind, // 种类
    pub base: Option<Box<Type>>, // 指向的类型
}
// step20预留
// // Pointer type that points to the base class.
// fn pointer_to(base: Option<Box<Type>>) -> Type {
//     Type { 
//         base,
//         kind: TypeKind::Ptr 
//     }
// }
// 为节点内的所有节点添加类型

impl Type{
    pub fn new(kind: TypeKind) -> Self {
        Self { kind, base: None }
    }
    // 半废弃，不如is_pointer来的方便，并且这个函数在new_sub中无法通过形如*(&y-1)的运算，
    // 也就是左侧是指针类型，右侧是Int类型，但是base_ty会把左侧判否
    // ,后续考虑删掉这个函数
    pub fn base_ty(&self) -> Option<&Self> {
        match &self.base {
            Some(ty) => ty.base_ty(),
            None => None,
        }
    }
    // ==== 匆忙放进impl的野函数，没有用self =============
    // ===============================================
    // 判断是否为整型
    pub fn is_integer(ty: &Type) -> bool {
        matches!(ty.kind, TypeKind::Int)
    }
    // 判断是否为指针
    pub fn is_pointer(ty: &Type) -> bool{
        matches!(ty.kind, TypeKind::Ptr)
    }
    /*======================================================================
                                add_type 类型推理核心函数
    =======================================================================*/
    pub fn add_type(mut n: &mut Box<Node>)-> Box<Node> {
        if n.ty.is_none() {
            // 递归访问所有节点以增加类型
            if let Some(lhs) = n.lhs.clone().as_mut() {
                n.lhs = Some(Type::add_type(lhs));
            }
            if let Some(rhs) = n.rhs.clone().as_mut() {
                n.rhs = Some(Type::add_type(rhs));
            }
            if let Some(cond) = n.cond.clone().as_mut() {
                Type::add_type(cond);
            }
            if let Some(then) = n.then.clone().as_mut() {
                Type::add_type(then);
            }
            if let Some(els) = n.els.clone().as_mut() {
                Type::add_type(els);
            }
            if let Some(init) = n.init.clone().as_mut() {
                Type::add_type(init);
            }
            if let Some(inc) = n.inc.clone().as_mut() {
                Type::add_type(inc);
            }
            // 访问链表内的所有节点以增加类型
            let mut body = n.body.clone();
            while let Some(mut node) = body {
                let mut node = Type::add_type(&mut node);
                body = node.next.take();
            }
            // 设置节点类型
            match n.kind {
                // 将节点类型设为 节点左部的类型
                NodeKind::NdAdd | NodeKind::NdSub | NodeKind::NdMul | NodeKind::NdDiv | NodeKind::NdNeg | 
                NodeKind::NdAssign => {
                    n.ty = n.lhs.as_ref().and_then(|lhs| lhs.ty.clone());
                    //let node = n;
                    n.clone()
                }
                // 将节点类型设为 int
                NodeKind::NdEq | NodeKind::NdNeq | NodeKind::NdLt | NodeKind::NdLe | NodeKind::NdVar |
                NodeKind::NdNum => {
                    n.ty = Some(Type {
                        kind: TypeKind::Int,
                        base: None,
                    });
                    n.clone()
                }
                // 将节点类型设为 指针，并指向左部的类型
                NodeKind::NdAddr => {
                    n.ty = n.lhs.as_ref().and_then(|lhs| {
                        Some(Type {
                            kind: TypeKind::Ptr,
                            base: Some(Box::new(lhs.ty.as_ref().unwrap().clone())),
                        })
                    });
                    n.clone()
                }
                // 节点类型：如果解引用指向的是指针，则为指针指向的类型；否则为int
                NodeKind::NdDeref => {
                    if let Some(lhs_ty) = n.lhs.as_ref().and_then(|lhs| lhs.ty.as_ref()) {
                        n.ty = Some(if lhs_ty.kind == TypeKind::Ptr {
                            *lhs_ty.base.as_ref().unwrap().clone()
                        } 
                    else {
                            Type {
                                kind: TypeKind::Int,
                                base: None,
                            }
                        });
                    }
                    n.clone()
                }
                // 其他情况不设置类型
                _ => {n.clone()}
            }
        }else{
            n.clone()
        }
    }
}

