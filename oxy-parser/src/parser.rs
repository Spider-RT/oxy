use crate::{
    Lexer,
    ast::{
        AstNode,
        Operator,
        FnArg,
    },
    types::OxyTy,
    tokens::{
        Token,
        TokenKind,
    },
};

pub struct AstParser<'a> {
    input: &'a [u8],
    lexer: Lexer,
}

impl<'a> AstParser<'a> {
    pub fn parse(mut self) -> Result<AstNode<'a>, AstParseError> {
        let mut program = Vec::new();
        loop {
            let token = match self.lexer.next(self.input) {
                Some(t) => t,
                None => break,
            };

            // Assumes depth 0
            match token.kind {
                TokenKind::Text => {
                    self.lexer.expect(self.input, TokenKind::DoubleColon)?;
                    let name = self.lexer.get_current(self.input);
                    let token = match self.lexer.next(self.input) {
                        Some(t) => t,
                        None => break,
                    };

                    match token.kind {
                        TokenKind::Text => program.push(self.parse_func()?),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        todo!()
    }

    fn parse_func(&mut self) -> Result<AstNode<'a>, AstParseError> {

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
