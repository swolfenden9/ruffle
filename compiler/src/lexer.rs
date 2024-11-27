//! Contains utils for lexing a Ruffle source file into a string of tokens.

use logos::Logos;
use std::{
    error::Error,
    fmt::{Debug, Display},
    num::{ParseFloatError, ParseIntError},
    ops::Range,
};

use crate::utils::rows_cols_index;

pub type Span = Range<usize>;

/// Wraps a token with its string slice and span in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct SlicedToken<'a> {
    pub token: Token,
    pub span: Span,
    pub source: &'a str,
}

impl<'a> SlicedToken<'a> {
    pub fn slice(&self) -> &str {
        &self.source[self.span.clone()]
    }
}

impl<'a> Display for SlicedToken<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

/// Wraps an error with its string slice and span in the source code.
#[derive(Debug, Clone, PartialEq)]
pub struct SlicedError<'a> {
    pub error: LexingError,
    pub span: Span,
    pub source: &'a str, // Reference to the full source string
}

impl<'a> SlicedError<'a> {
    pub fn slice(&self) -> &str {
        &self.source[self.span.clone()]
    }
}

impl<'a> Display for SlicedError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rows, cols) = rows_cols_index(self.source, self.span.start);
        writeln!(f, "error at {}:{}:", rows, cols)?;
        writeln!(f, "{}", self.slice())?;
        writeln!(f, "^ {}", self.error)?;
        Ok(())
    }
}

/// Error type returned from lexing.
#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    #[default]
    NonAsciiCharacter,
    InvalidInteger(&'static str),
    InvalidFloat,
}

impl Error for LexingError {}

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::NonAsciiCharacter => write!(f, "non ascii character"),
            LexingError::InvalidInteger(err) => write!(f, "invalid integer: {}", err),
            LexingError::InvalidFloat => write!(f, "invalid float"),
        }
    }
}

/// Error type returned by calling `lex.slice().parse()` to i32.
impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow"),
            _ => LexingError::InvalidInteger("other"),
        }
    }
}

/// Error type returned by calling `lex.slice().parse()` to f32.
impl From<ParseFloatError> for LexingError {
    fn from(_err: ParseFloatError) -> Self {
        Self::InvalidFloat
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+|//.*|/\*([^*]|\*+[^*/])*\*+/")] // Comments
#[logos(error = LexingError)]
pub enum Token {
    // Symbols
    #[token(".")]
    Period,
    #[token(",")]
    Comma,
    #[token(";")]
    Semi,
    #[token("!")]
    Bang,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,
    #[token("::")]
    ColonColon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("->")]
    StraightArrow,
    #[token("=>")]
    EqArrow,

    // Arithmetic Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Modulus,

    // Comparison Operators
    #[token("==")]
    EqEq,
    #[token("===")]
    EqEqEq,
    #[token("!=")]
    Ne,
    #[token("!==")]
    Nee,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEq,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEq,

    // Logical Operators
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,

    // Assignment Operators
    #[token("=")]
    Eq,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    StarEq,
    #[token("/=")]
    SlashEq,

    // Keywords
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("return")]
    Return,
    #[token("class")]
    Class,
    #[token("impl")]
    Impl,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("self")]
    SelfValue,
    #[token("super")]
    Super,
    #[token("use")]
    Use,
    #[token("mod")]
    Mod,
    #[token("const")]
    Const,
    #[token("static")]
    Static,

    // Literals
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Integer(i32),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f32),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len() - 1].to_owned())]
    String(String),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Ident(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Symbols
                Token::Period => ".",
                Token::Comma => ",",
                Token::Semi => ";",
                Token::Bang => "!",
                Token::Question => "?",
                Token::Colon => ":",
                Token::ColonColon => "::",
                Token::LParen => "(",
                Token::RParen => ")",
                Token::LSquare => "[",
                Token::RSquare => "]",
                Token::LBrace => "{",
                Token::RBrace => "}",
                Token::StraightArrow => "->",
                Token::EqArrow => "=>",

                // Arithmetic Operators
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Star => "*",
                Token::Slash => "/",
                Token::Modulus => "%",

                // Comparison Operators
                Token::EqEq => "==",
                Token::EqEqEq => "===",
                Token::Ne => "!=",
                Token::Nee => "!==",
                Token::Less => "<",
                Token::LessEq => "<=",
                Token::Greater => ">",
                Token::GreaterEq => ">=",

                // Logical Operators
                Token::AndAnd => "&&",
                Token::OrOr => "||",

                // Assignment Operators
                Token::Eq => "=",
                Token::PlusEq => "+=",
                Token::MinusEq => "-=",
                Token::StarEq => "*=",
                Token::SlashEq => "/=",

                // Keywords
                Token::Let => "let",
                Token::Fn => "fn",
                Token::If => "if",
                Token::Else => "else",
                Token::While => "while",
                Token::For => "for",
                Token::Return => "return",
                Token::Class => "class",
                Token::Impl => "impl",
                Token::Struct => "struct",
                Token::Enum => "enum",
                Token::SelfValue => "self",
                Token::Super => "super",
                Token::Use => "use",
                Token::Mod => "mod",
                Token::Const => "const",
                Token::Static => "static",

                // Literals
                Token::Integer(value) => return write!(f, "{}", value),
                Token::Float(value) => return write!(f, "{}", value),
                Token::String(value) => return write!(f, "str(\"{}\")", value),
                Token::Ident(value) => return write!(f, "ident({})", value),
            }
        )
    }
}

/// Lexes a source file into tokens with span information.
pub fn lex_source(source: &str) -> Vec<Result<SlicedToken, SlicedError>> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        let span = lexer.span();

        tokens.push(match token {
            Ok(t) => Ok(SlicedToken {
                token: t,
                span,
                source,
            }),
            Err(e) => Err(SlicedError {
                error: e,
                span,
                source,
            }),
        });
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_valid_tokens() {
        let source = r#"let x = 42; let y = 3.14; return x + y;"#;
        let tokens = lex_source(source);

        assert_eq!(tokens.len(), 15);
        assert!(tokens.iter().all(|t| t.is_ok()));

        let expected = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Eq,
            Token::Integer(42),
            Token::Semi,
            Token::Let,
            Token::Ident("y".to_string()),
            Token::Eq,
            Token::Float(3.14),
            Token::Semi,
            Token::Return,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semi,
        ];

        for (token, expected) in tokens.into_iter().map(Result::unwrap).zip(expected) {
            assert_eq!(token.token, expected);
        }
    }

    // #[test]
    // fn test_lex_invalid_integer() {
    //     let source = "let x = 1000000000000000000000000000;";
    //     let tokens = lex_source(source);

    //     assert_eq!(tokens.len(), 5);

    //     assert_eq!(tokens[0].as_ref().unwrap().token, Token::Let);
    //     assert!(matches!(
    //         tokens[3],
    //         Err(SlicedError {
    //             error: LexingError::InvalidInteger(_),
    //             ..
    //         })
    //     ));
    // }

    // #[test]
    // fn test_lex_invalid_float() {
    //     let source = "let pi = 3.14.15;";
    //     let tokens = lex_source(source);

    //     assert_eq!(tokens.len(), 6);

    //     assert_eq!(tokens[0].as_ref().unwrap().token, Token::Let);
    //     assert!(matches!(
    //         tokens[3],
    //         Err(SlicedError {
    //             error: LexingError::InvalidFloat,
    //             ..
    //         })
    //     ));
    // }

    #[test]
    fn test_lex_unexpected_character() {
        let source = "let x = @;";
        let tokens = lex_source(source);

        assert_eq!(tokens.len(), 5);

        assert_eq!(tokens[0].as_ref().unwrap().token, Token::Let);
        assert!(matches!(
            tokens[3],
            Err(SlicedError {
                error: LexingError::NonAsciiCharacter,
                ..
            })
        ));
    }

    #[test]
    fn test_lex_keywords_and_identifiers() {
        let source = "fn foo() { let bar = 42; }";
        let tokens = lex_source(source);

        assert_eq!(tokens.len(), 11);
        assert!(tokens.iter().all(|t| t.is_ok()));

        let expected = vec![
            Token::Fn,
            Token::Ident("foo".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Let,
            Token::Ident("bar".to_string()),
            Token::Eq,
            Token::Integer(42),
            Token::Semi,
            Token::RBrace,
        ];

        for (token, expected) in tokens.into_iter().map(Result::unwrap).zip(expected) {
            assert_eq!(token.token, expected);
        }
    }

    #[test]
    fn test_lex_strings() {
        let source = r#"let greeting = "Hello, World!";"#;
        let tokens = lex_source(source);

        assert_eq!(tokens.len(), 5);
        assert!(tokens.iter().all(|t| t.is_ok()));

        let expected = vec![
            Token::Let,
            Token::Ident("greeting".to_string()),
            Token::Eq,
            Token::String("Hello, World!".to_string()),
            Token::Semi,
        ];

        for (token, expected) in tokens.into_iter().map(Result::unwrap).zip(expected) {
            assert_eq!(token.token, expected);
        }
    }

    #[test]
    fn test_lex_nested_expressions() {
        let source = "let result = (1 + 2) * (3 - 4);";
        let tokens = lex_source(source);

        assert_eq!(tokens.len(), 15);
        assert!(tokens.iter().all(|t| t.is_ok()));

        let expected = vec![
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Eq,
            Token::LParen,
            Token::Integer(1),
            Token::Plus,
            Token::Integer(2),
            Token::RParen,
            Token::Star,
            Token::LParen,
            Token::Integer(3),
            Token::Minus,
            Token::Integer(4),
            Token::RParen,
            Token::Semi,
        ];

        for (token, expected) in tokens.into_iter().map(Result::unwrap).zip(expected) {
            assert_eq!(token.token, expected);
        }
    }

    #[test]
    fn test_empty_source() {
        let source = "";
        let tokens = lex_source(source);

        assert!(tokens.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let source = "   \n\t   ";
        let tokens = lex_source(source);

        assert!(tokens.is_empty());
    }

    #[test]
    fn test_comments_skipped() {
        // TODO: Multi-line comments
        let source = r#"
            // This is a single-line comment
            let x = 10;
        "#;
        let tokens = lex_source(source);

        assert_eq!(tokens.len(), 5);

        let expected = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Eq,
            Token::Integer(10),
            Token::Semi,
        ];

        for (token, expected) in tokens.into_iter().map(Result::unwrap).zip(expected) {
            assert_eq!(token.token, expected);
        }
    }
}
