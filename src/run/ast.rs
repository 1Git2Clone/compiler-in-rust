use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ast {
    Int(i32),
    BinOp {
        lhs: Box<Ast>,
        op: Token,
        rhs: Box<Ast>,
    },
}
