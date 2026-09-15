use crate::{
    Lexer,
    tokens::{Token, TokenKind},
    types::OxyTy,
};

pub enum Definition<'a> {
    Function {
        name: &'a [u8],
        args: Vec<FnArg<'a>>,
        return_ty: Option<OxyTy>,
        body: Vec<Expression<'a>>,
    },

    LetBinding {
        name: &'a [u8],
        expression: Expression<'a>,
    },
}

pub struct FnArg<'a> {
    ty: OxyTy,
    name: &'a [u8],
}

pub enum Expression<'a> {
    FunctionCall {
        name: &'a [u8],
        args: Vec<Expression<'a>>,
    },

    Math {
        lhs: Box<Expression<'a>>,
        rhs: Box<Expression<'a>>,
        operation: MathOp,
    },

    RawValue {
        ty: OxyTy,
        value: &'a [u8]
    },

    Reference {
        ty: OxyTy,
        name: &'a [u8],
    },

    Return {
        expr: Box<Expression<'a>>,
    },
}

pub enum MathOp {
    Plus,
    Minus,
    Divide,
    Multiply
}

pub struct Ast<'a> {
    definitions: Vec<Definition<'a>>,
}

impl<'a> Ast<'a> {
    pub fn parse(&mut self, input: &'a [u8]) -> Result<(), AstParseError> {
        let mut lexer = Lexer::default();

        while let Some(token) = lexer.next(input) {
            match token.kind {
                TokenKind::Text => {
                    lexer.expect(input, TokenKind::DoubleColon, AstParseError::ExpectedSymbol(TokenKind::DoubleColon))?;

                    let text = &input[lexer.get_ref_range()];

                    if let Some(token) = lexer.next(input) {
                        match token.kind {
                            TokenKind::Fn => self.parse_fn_def(input, text, &mut lexer)?,
                            _ => {}
                        }
                    } else {
                        return Err(AstParseError::ExpectedDefinition);
                    }
                },
                _ => {}
            }
        }

        todo!()
    }

    fn parse_fn_def(&mut self, input: &'a [u8], fn_name: &'a [u8], lexer: &mut Lexer) -> Result<(), AstParseError> {
        lexer.expect(input, TokenKind::LParen, AstParseError::ExpectedSymbol(TokenKind::LParen))?;
        let mut args = Vec::new();

        while let Some(arg) = self.parse_fn_def_arg(input, lexer) {
            args.push(arg);
        }
        todo!()
    }

    fn parse_fn_def_arg(&mut self, input: &'a [u8], lexer: &mut Lexer) -> Option<FnArg<'a>> {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AstParseError {
    #[error("Expected symbol: {0}")]
    ExpectedSymbol(TokenKind),

    #[error("Expected Definition like fn, struct, enum...")]
    ExpectedDefinition,
}
