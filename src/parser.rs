use anyhow::{bail, Result};

use crate::ast::{Exp, FunctionDefinition, Program, Statement};
use crate::tokens::Token;

/// Parser error with a descriptive message.
#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

/// A peekable token stream for the parser.
struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Peek at the next token without consuming it.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Consume and return the next token.
    fn take(&mut self) -> Result<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Ok(tok)
        } else {
            bail!(ParseError("Unexpected end of input".to_string()));
        }
    }

    /// Consume the next token and verify it matches the expected token.
    fn expect(&mut self, expected: &Token) -> Result<()> {
        let actual = self.take()?;
        if &actual != expected {
            bail!(ParseError(format!(
                "Expected {} but found {}",
                expected, actual
            )));
        }
        Ok(())
    }

    /// Returns true if all tokens have been consumed.
    fn is_empty(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

/// Parse an `<identifier>` — expects an Identifier token.
fn parse_identifier(stream: &mut TokenStream) -> Result<String> {
    match stream.take()? {
        Token::Identifier(name) => Ok(name),
        other => bail!(ParseError(format!(
            "Expected an identifier but found {}",
            other
        ))),
    }
}

/// Parse `<exp>` ::= `<int>`
fn parse_exp(stream: &mut TokenStream) -> Result<Exp> {
    match stream.take()? {
        Token::Constant(val) => Ok(Exp::Constant(val)),
        other => bail!(ParseError(format!(
            "Expected a constant but found {}",
            other
        ))),
    }
}

/// Parse `<statement>` ::= "return" `<exp>` ";"
fn parse_statement(stream: &mut TokenStream) -> Result<Statement> {
    stream.expect(&Token::KWReturn)?;
    let exp = parse_exp(stream)?;
    stream.expect(&Token::Semicolon)?;
    Ok(Statement::Return(exp))
}

/// Parse `<function>` ::= "int" `<identifier>` "(" "void" ")" "{" `<statement>` "}"
fn parse_function(stream: &mut TokenStream) -> Result<FunctionDefinition> {
    stream.expect(&Token::KWInt)?;
    let name = parse_identifier(stream)?;
    stream.expect(&Token::OpenParen)?;
    stream.expect(&Token::KWVoid)?;
    stream.expect(&Token::CloseParen)?;
    stream.expect(&Token::OpenBrace)?;
    let body = parse_statement(stream)?;
    stream.expect(&Token::CloseBrace)?;
    Ok(FunctionDefinition { name, body })
}

/// Parse `<program>` ::= `<function>`
fn parse_program(stream: &mut TokenStream) -> Result<Program> {
    let function = parse_function(stream)?;
    if !stream.is_empty() {
        bail!(ParseError(
            "Unexpected tokens after function definition".to_string()
        ));
    }
    Ok(Program { function })
}

/// Public entry point: parse a vector of tokens into an AST.
pub fn parse(tokens: Vec<Token>) -> Result<Program> {
    let mut stream = TokenStream::new(tokens);
    parse_program(&mut stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_program() {
        let tokens = vec![
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
        ];
        let program = parse(tokens).unwrap();
        assert_eq!(program.function.name, "main");
        assert_eq!(program.function.body, Statement::Return(Exp::Constant(42)));
    }

    #[test]
    fn parse_error_missing_semicolon() {
        let tokens = vec![
            Token::KWInt,
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::KWVoid,
            Token::CloseParen,
            Token::OpenBrace,
            Token::KWReturn,
            Token::Constant(0),
            // missing semicolon
            Token::CloseBrace,
        ];
        assert!(parse(tokens).is_err());
    }
}
