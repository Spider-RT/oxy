use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    #[cfg(test)]
    pub str: String,
}

#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum TokenKind {
    Text,
    Number,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    Colon,
    Semicolon,
    Comma,
    Dot,
    Slash,
    LessThan,
    GreaterThan,
    Minus,
    Plus,
    Asterisk,
    Ampersand,
    Equal,
    Underscore,
    Quote,
    Tick,
    Newline,
    ExclamationMark,

    // Doubles
    DoubleColon,
    DoubleSlash,
    DoubleEqual,

    //Keywords
    Fn,
    Struct,
    Enum,
    Union,
    Use,
    Impl,
    If,
    Else,
    Return,
    Let,
}

impl From<&[u8]> for TokenKind {
    fn from(s: &[u8]) -> Self {
        match s {
            b"(" => TokenKind::LParen,
            b")" => TokenKind::RParen,
            b"[" => TokenKind::LBracket,
            b"]" => TokenKind::RBracket,
            b"{" => TokenKind::LCurly,
            b"}" => TokenKind::RCurly,
            b":" => TokenKind::Colon,
            b";" => TokenKind::Semicolon,
            b"," => TokenKind::Comma,
            b"." => TokenKind::Dot,
            b"/" => TokenKind::Slash,
            b"<" => TokenKind::LessThan,
            b">" => TokenKind::GreaterThan,
            b"-" => TokenKind::Minus,
            b"+" => TokenKind::Plus,
            b"*" => TokenKind::Asterisk,
            b"&" => TokenKind::Ampersand,
            b"=" => TokenKind::Equal,
            b"_" => TokenKind::Underscore,
            br#"""# => TokenKind::Quote,
            b"'" => TokenKind::Tick,
            b"\n" => TokenKind::Newline,
            b"!" => TokenKind::ExclamationMark,

            // Doubles
            b"::" => TokenKind::DoubleColon,
            b"//" => TokenKind::DoubleSlash,
            b"==" => TokenKind::DoubleEqual,

            // Keywords
            b"fn" => TokenKind::Fn,
            b"struct" => TokenKind::Struct,
            b"enum" => TokenKind::Enum,
            b"union" => TokenKind::Union,
            b"use" => TokenKind::Use,
            b"impl" => TokenKind::Impl,
            b"return" => TokenKind::Return,
            b"if" => TokenKind::If,
            b"else" => TokenKind::Else,
            b"let" => TokenKind::Let,
            _ => TokenKind::Text,
        }
    }
}
