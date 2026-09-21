use crate::{
    Lexer,
    tokens::{Token, TokenKind},
    types::OxyTy,
};

pub enum AstNode<'a> {
    Program {
        nodes: Vec<AstNode<'a>>,
    },

    Function {
        name: &'a [u8],
        args: Vec<FnArg<'a>>,
        ret: OxyTy,
        body: Vec<AstNode<'a>>,
    },

    FunctionCall {
        name: &'a [u8],
        args: Vec<AstNode<'a>>,
    },

    LetBinding {
        name: &'a [u8],
        ty: OxyTy,
        expr: Box<AstNode<'a>>,
    },

    MathOp {
        lhs: Box<AstNode<'a>>,
        rhs: Box<AstNode<'a>>,
        operator: Operator,
    },

    Tuple {
        expr: Vec<AstNode<'a>>,
    }
}

pub struct FnArg<'a> {
    name: &'a [u8],
    ty: OxyTy,
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
