#![allow(unused)]

use crate::{parser::AstParseError, tokens::{Token, TokenKind}};
use std::{ops::Range, str::FromStr};

pub mod ast;
pub mod parser;
pub mod tokens;
pub mod types;

#[derive(Default)]
pub struct Lexer {
    start: usize,
    end: usize,
}

impl Lexer {
    pub fn get_ref_range(&self) -> Range<usize> {
        self.start..self.end
    }

    // Checks the next token and moves forward
    pub fn expect(&mut self, input: &[u8], expect: TokenKind) -> Result<(), AstParseError> {
        if !self.next(input).is_some_and(|t| t.kind == expect) {
            return Err(AstParseError::ExpectedSymbol(expect))
        }
        Ok(())
    } 

    pub fn get_current<'a>(&self, input: &'a [u8]) -> &'a [u8] {
        &input[self.start..self.end]
    }

    pub fn look_ahead(&self, input: &[u8], distance: usize) -> Vec<Option<Token>> {
        let mut tokens = Vec::new();
        let mut start = self.start;
        let mut end = self.end;


        for _ in 0..distance {
            start = end;
            let t = Self::next_inner(&mut start, &mut end, input).map(|_| Self::current_token(start, end, input));
            tokens.push(t);
        }

        tokens
    }

    fn peek(end: usize, input: &[u8]) -> Option<u8> {
        let peek = end + 1;
        if peek >= input.len() {
            return None;
        }

        Some(input[peek])
    }

    fn next_inner(start: &mut usize, end: &mut usize, input: &[u8]) -> Option<()> {
        loop {
            if *end >= input.len() {
                *end = end.saturating_sub(1);
                return None;
            }

            let ch = input[*end] as char;
            if ch.is_ascii_whitespace() {
                if ch == '\n' {
                    if *start == *end {
                        break;
                    }
                    *end -= 1;
                    break;
                }
                if *start == *end {
                    *end += 1;
                    *start += 1;
                    continue;
                } else {
                    *end -= 1;
                    break;
                }
            }

            if ch.is_ascii_punctuation() {
                if *start == *end {
                    if [':', '/', '='].contains(&ch) && Self::peek(*end, input).is_some_and(|c| c == ch as u8) {
                        *end += 1;
                    }

                    break;
                }
                *end -= 1;
                break;
            }

            *end += 1;
        }

        Some(())

    }

    fn current_token(start: usize, end: usize, input: &[u8]) -> Token {
        let bytes = &input[start..=end];
        let kind = TokenKind::from(bytes);

        Token {
            kind,
            #[cfg(test)]
            str: String::from_utf8_lossy(bytes).to_string(),
        }
    }

    pub fn next(&mut self, input: &[u8]) -> Option<Token> {
        self.start = self.end;

        Self::next_inner(&mut self.start, &mut self.end, input)?;

        let token = Self::current_token(self.start, self.end, input);

        self.end += 1;

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{Token, TokenKind};

    struct LexerTest<'a> {
        input: &'a [u8],
        lexer: Lexer,
    }

    impl<'a> LexerTest<'a> {
        fn new(input: &'a [u8]) -> Self {
            Self {
                input,
                lexer: Lexer::default(),
            }
        }
    }

    impl<'a> Iterator for LexerTest<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            self.lexer.next(self.input)
        }
    }

    #[test]
    fn parse_fn_item() {
        let input = "add :: func(a: u16, b: u16) u16 {\n return a + b; \n}";
        let lexer = LexerTest::new(input.as_bytes());

        let expected: Vec<Token> = [
            (TokenKind::Text, "add"),
            (TokenKind::DoubleColon, "::"),
            (TokenKind::Func, "func"),
            (TokenKind::LParen, "("),
            (TokenKind::Text, "a"),
            (TokenKind::Colon, ":"),
            (TokenKind::Text, "u16"),
            (TokenKind::Comma, ","),
            (TokenKind::Text, "b"),
            (TokenKind::Colon, ":"),
            (TokenKind::Text, "u16"),
            (TokenKind::RParen, ")"),
            (TokenKind::Text, "u16"),
            (TokenKind::LCurly, "{"),
            (TokenKind::Newline, "\n"),
            (TokenKind::Return, "return"),
            (TokenKind::Text, "a"),
            (TokenKind::Plus, "+"),
            (TokenKind::Text, "b"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Newline, "\n"),
            (TokenKind::RCurly, "}"),
        ]
        .into_iter()
        .map(|(kind, str)| Token {
            kind,
            str: str.to_string(),
        })
        .collect();

        assert_eq!(expected, lexer.collect::<Vec<Token>>());
    }

    #[test]
    fn parse_struct_item() {
        let input = b"Foo :: struct {\nx: u8,\ny: u8\n}\n";
        let lexer = LexerTest::new(input);

        let expected: Vec<Token> = [
            (TokenKind::Text, "Foo"),
            (TokenKind::DoubleColon, "::"),
            (TokenKind::Struct, "struct"),
            (TokenKind::LCurly, "{"),
            (TokenKind::Newline, "\n"),
            (TokenKind::Text, "x"),
            (TokenKind::Colon, ":"),
            (TokenKind::Text, "u8"),
            (TokenKind::Comma, ","),
            (TokenKind::Newline, "\n"),
            (TokenKind::Text, "y"),
            (TokenKind::Colon, ":"),
            (TokenKind::Text, "u8"),
            (TokenKind::Newline, "\n"),
            (TokenKind::RCurly, "}"),
            (TokenKind::Newline, "\n"),
        ]
        .into_iter()
        .map(|(kind, str)| Token {
            kind,
            str: String::from_utf8_lossy(str.as_bytes()).to_string(),
        })
        .collect();

        assert_eq!(expected, lexer.collect::<Vec<Token>>());
    }

    #[test]
    fn parse_enum_item() {
        let input = b"Foo :: enum {\nIoError,\nMapError\n}\n";
        let lexer = LexerTest::new(input);

        let expected: Vec<Token> = [
            (TokenKind::Text, "Foo"),
            (TokenKind::DoubleColon, "::"),
            (TokenKind::Enum, "enum"),
            (TokenKind::LCurly, "{"),
            (TokenKind::Newline, "\n"),
            (TokenKind::Text, "IoError"),
            (TokenKind::Comma, ","),
            (TokenKind::Newline, "\n"),
            (TokenKind::Text, "MapError"),
            (TokenKind::Newline, "\n"),
            (TokenKind::RCurly, "}"),
            (TokenKind::Newline, "\n"),
        ]
        .into_iter()
        .map(|(kind, str)| Token {
            kind,
            str: String::from_utf8_lossy(str.as_bytes()).to_string(),
        })
        .collect();

        assert_eq!(expected, lexer.collect::<Vec<Token>>());
    }

    #[test]
    fn parse_comment_item() {
        let input = b"Foo :: struct { // Foo\nx: u8}";
        let lexer = LexerTest::new(input);

        let expected: Vec<Token> = [
            (TokenKind::Text, "Foo"),
            (TokenKind::DoubleColon, "::"),
            (TokenKind::Struct, "struct"),
            (TokenKind::LCurly, "{"),
            (TokenKind::DoubleSlash, "//"),
            (TokenKind::Text, "Foo"),
            (TokenKind::Newline, "\n"),
            (TokenKind::Text, "x"),
            (TokenKind::Colon, ":"),
            (TokenKind::Text, "u8"),
            (TokenKind::RCurly, "}"),
        ]
        .into_iter()
        .map(|(kind, str)| Token {
            kind,
            str: String::from_utf8_lossy(str.as_bytes()).to_string(),
        })
        .collect();

        assert_eq!(expected, lexer.collect::<Vec<Token>>());
    }

    #[test]
    fn skip_empty_input() {
        let input = "";
        let lexer = LexerTest::new(input.as_bytes());
        assert!(lexer.collect::<Vec<Token>>().is_empty())
    }

    #[test]
    fn lexer_look_ahead() {
        let input = "add :: func".as_bytes();
        let mut lexer = Lexer::default();

        assert_eq!(Some(Token {
            kind: TokenKind::Text,
            str: String::from("add"),
        }), lexer.next(&input));

        let look_ahead_vec = lexer.look_ahead(&input, 3);
        let expected = vec![
            Some(Token{
                kind: TokenKind::DoubleColon,
                str: String::from("::"),
            }),
            Some(Token {
                kind: TokenKind::Func,
                str: String::from("func"),
            }),
            None
        ];

        assert_eq!(expected, look_ahead_vec)
    }
}
