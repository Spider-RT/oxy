#![allow(unused)]

use crate::{ast::AstParseError, tokens::{Token, TokenKind}};
use std::{ops::Range, str::FromStr};

pub mod ast;
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

    pub fn expect(&mut self, input: &[u8], expect: TokenKind, on_err: AstParseError) -> Result<(), AstParseError> {
        if !self.next(input).is_some_and(|t| t.kind == expect) {
            return Err(on_err)
        }
        Ok(())
    } 

    fn peek(&self, input: &[u8]) -> Option<u8> {
        let peek = self.end + 1;
        if peek >= input.len() {
            return None;
        }

        Some(input[peek])
    }

    pub fn next(&mut self, input: &[u8]) -> Option<Token> {
        self.start = self.end;

        loop {
            if self.end >= input.len() {
                self.end = self.end.saturating_sub(1);
                return None;
            }

            let ch = input[self.end] as char;
            if ch.is_ascii_whitespace() {
                if ch == '\n' {
                    if self.start == self.end {
                        break;
                    }
                    self.end -= 1;
                    break;
                }
                if self.start == self.end {
                    self.end += 1;
                    self.start += 1;
                    continue;
                } else {
                    self.end -= 1;
                    break;
                }
            }

            if ch.is_ascii_punctuation() {
                if self.start == self.end {
                    if [':', '/', '='].contains(&ch) && self.peek(input).is_some_and(|c| c == ch as u8) {
                        self.end += 1;
                    }

                    break;
                }
                self.end -= 1;
                break;
            }

            self.end += 1;
        }

        let bytes = &input[self.start..=self.end];
        let kind = TokenKind::from(bytes);

        self.end += 1;

        Some(Token {
            kind,
            #[cfg(test)]
            str: String::from_utf8_lossy(bytes).to_string(),
        })

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
        let input = "add :: fn(a: u8, b: u8) u8 {\nreturn a + b;\n}";
        let lexer = LexerTest::new(input.as_bytes());

        let expected: Vec<Token> = [
            (TokenKind::Text, "add"),
            (TokenKind::DoubleColon, "::"),
            (TokenKind::Fn, "fn"),
            (TokenKind::LParen, "("),
            (TokenKind::Text, "a"),
            (TokenKind::Colon, ":"),
            (TokenKind::Text, "u8"),
            (TokenKind::Comma, ","),
            (TokenKind::Text, "b"),
            (TokenKind::Colon, ":"),
            (TokenKind::Text, "u8"),
            (TokenKind::RParen, ")"),
            (TokenKind::Text, "u8"),
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
    fn parse_union_item() {
        let input = b"Foo :: union {\nIoError,\nMapError\n}\n";
        let lexer = LexerTest::new(input);

        let expected: Vec<Token> = [
            (TokenKind::Text, "Foo"),
            (TokenKind::DoubleColon, "::"),
            (TokenKind::Union, "union"),
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
}
