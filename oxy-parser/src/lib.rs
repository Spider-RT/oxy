#![allow(unused)]

use crate::tokens::{Token, TokenKind};
use std::{ops::Range, str::FromStr};

pub mod tokens;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Expected {0}")]
    ExpectedSymbol(TokenKind),
}

#[derive(Default)]
pub struct LexerCounter {
    start: usize,
    end: usize,
}

impl LexerCounter {
    #[inline(always)]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[inline(always)]
    pub fn advance(&mut self) {
        self.start += 1;
        self.end += 1;
    }

    #[inline(always)]
    pub fn has_moved(&self) -> bool {
        self.start != self.end
    }

    #[inline(always)]
    pub fn align(&mut self) {
        self.start = self.end;
    }
}

pub struct Lexer<'a> {
    counter: LexerCounter,
    input: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            counter: LexerCounter::default(),
        }
    }

    pub fn next_is_same(&self, pattern: &u8) -> bool {
        self.input.get(self.counter.end + 1).is_some_and(|next| next == pattern)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter.align();

        loop {
            if self.counter.end >= self.input.len() {
                if !self.counter.has_moved() {
                    return None;
                }

                break;
            }

            let ch = self.input[self.counter.end];

            if ch.is_ascii_whitespace() && ch != b'\n' {
                if !self.counter.has_moved() {
                    self.counter.advance();
                    continue;
                }

                break;
            }

            if !ch.is_ascii_alphanumeric() {
                if !self.counter.has_moved() {
                    if self.next_is_same(&ch) {
                        self.counter.end += 2;
                        break;
                    }

                    self.counter.end += 1;
                    break;
                }

                break;
            }
            
            self.counter.end += 1;
        }

        let slice = &self.input[self.counter.range()];
        let kind = TokenKind::from(slice);

        let token = Token {
            kind,
            #[cfg(test)]
            str: String::from_utf8_lossy(slice).to_string(),
        };

        return Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::{Token, TokenKind};
    use pretty_assertions::assert_eq;

    #[test]
    fn lexer_single_item() {
        let input = "hello";
    }

    #[test]
    fn lexer_parse_fn_item() {
        let input = b"add :: func(a: u16, b: u16) u16 {\n return a + b; \n}";
        let lexer = Lexer::new(input);

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
    fn lexer_parse_struct_item() {
        let input = b"Foo :: struct {\nx: u8,\ny: u8\n}\n";
        let lexer = Lexer::new(input);

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
    fn lexer_parse_enum_item() {
        let input = b"Foo :: enum {\nIoError,\nMapError\n}\n";
        let lexer = Lexer::new(input);

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
    fn lexer_parse_comment_item() {
        let input = b"Foo :: struct { // Foo\nx: u8}";
        let lexer = Lexer::new(input);

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
    fn lexer_skip_empty_input() {
        let input = "";
        let lexer = Lexer::new(input.as_bytes());
        assert!(lexer.collect::<Vec<Token>>().is_empty())
    }
}
