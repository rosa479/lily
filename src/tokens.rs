/// Token types for the C compiler (Chapter 1).
///
/// These represent every lexical element the compiler can recognize.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Tokens with values
    Identifier(String),
    Constant(i64),

    // Keywords
    KWInt,
    KWReturn,
    KWVoid,

    // Punctuation
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(name) => write!(f, "Identifier({})", name),
            Token::Constant(val) => write!(f, "Constant({})", val),
            Token::KWInt => write!(f, "int"),
            Token::KWReturn => write!(f, "return"),
            Token::KWVoid => write!(f, "void"),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::OpenBrace => write!(f, "{{"),
            Token::CloseBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
        }
    }
}
