use anyhow::{bail, Result};

use crate::tokens::Token;

/// Lexer error with a message indicating what went wrong.
#[derive(Debug)]
pub struct LexError(pub String);

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lex error: {}", self.0)
    }
}

impl std::error::Error for LexError {}

/// Lex the entire source string into a vector of tokens.
pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        // Skip whitespace
        if chars[pos].is_ascii_whitespace() {
            pos += 1;
            continue;
        }

        // Identifiers and keywords: [A-Za-z_][A-Za-z0-9_]*
        if chars[pos].is_ascii_alphabetic() || chars[pos] == '_' {
            let start = pos;
            while pos < chars.len() && (chars[pos].is_ascii_alphanumeric() || chars[pos] == '_') {
                pos += 1;
            }
            let word: String = chars[start..pos].iter().collect();
            let token = match word.as_str() {
                "int" => Token::KWInt,
                "return" => Token::KWReturn,
                "void" => Token::KWVoid,
                _ => Token::Identifier(word),
            };
            tokens.push(token);
            continue;
        }

        // Integer constants: [0-9]+
        if chars[pos].is_ascii_digit() {
            let start = pos;
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                pos += 1;
            }
            // If immediately followed by an identifier char, that's invalid (e.g. "123abc")
            if pos < chars.len()
                && (chars[pos].is_ascii_alphabetic() || chars[pos] == '_')
            {
                bail!(LexError(format!(
                    "Invalid token: number followed by identifier character at position {}",
                    start
                )));
            }
            let num_str: String = chars[start..pos].iter().collect();
            let value = num_str.parse::<i64>().map_err(|e| {
                LexError(format!("Failed to parse integer '{}': {}", num_str, e))
            })?;
            tokens.push(Token::Constant(value));
            continue;
        }

        // Single-character punctuation and operators
        let token = match chars[pos] {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ';' => Token::Semicolon,
            '~' => Token::Tilde,
            '-' => {
                // Check for "--" (double hyphen)
                if pos + 1 < chars.len() && chars[pos + 1] == '-' {
                    pos += 1; // consume the second '-'
                    Token::DoubleHyphen
                } else {
                    Token::Hyphen
                }
            }
            other => bail!(LexError(format!(
                "Unexpected character '{}' at position {}",
                other, pos
            ))),
        };
        tokens.push(token);
        pos += 1;
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_simple_program() {
        let source = "int main(void) { return 42; }";
        let tokens = lex(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::KWInt,
                Token::Identifier("main".to_string()),
                Token::OpenParen,
                Token::KWVoid,
                Token::CloseParen,
                Token::OpenBrace,
                Token::KWReturn,
                Token::Constant(42),
                Token::Semicolon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn lex_error_invalid_token() {
        let source = "123abc";
        assert!(lex(source).is_err());
    }

    #[test]
    fn lex_unary_operators() {
        let source = "return ~(-42);";
        let tokens = lex(source).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::KWReturn,
                Token::Tilde,
                Token::OpenParen,
                Token::Hyphen,
                Token::Constant(42),
                Token::CloseParen,
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn lex_double_hyphen() {
        let source = "--5";
        let tokens = lex(source).unwrap();
        assert_eq!(
            tokens,
            vec![Token::DoubleHyphen, Token::Constant(5)]
        );
    }
}
