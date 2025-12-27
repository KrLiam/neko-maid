//! This module implements the parsing functionality for NekoMaid UI files.
//! It provides functions to read and interpret `.neko_ui` files.

use crate::parse::nodes::ModuleNode;
use crate::parse::token::TokenPosition;

pub mod ast;
pub mod nodes;
pub mod token;

/// Parses a NekoMaid UI file from the given input string and returns the
/// resulting root AST node.
pub fn parse_neko_ui(input: &str) -> Result<ModuleNode, NekoMaidParseError> {
    let tokens = token::tokenize(input)?;
    let file = ast::build_ast(tokens)?;
    Ok(file)
}

/// Errors that can occur during parsing of NekoMaid UI files.
#[derive(Debug, thiserror::Error)]
pub enum NekoMaidParseError {
    /// Error during tokenization
    #[error("Tokenization error: {0}")]
    TokenizerError(#[from] token::TokenizeError),

    /// Unexpected token encountered
    #[error("Unexpected token at {position}: found {found}, expected one of: {expected:?}")]
    UnexpectedToken {
        /// The expected token list.
        expected: Vec<String>,

        /// The found token description.
        found: String,

        /// The position of the unexpected token.
        position: TokenPosition,
    },

    /// Unexpected end of input
    #[error("Unexpected end of input")]
    EndOfStream,

    /// An error that occurs due to a token storing an invalid value for its
    /// type. This is an internal error and should not occur during normal
    /// parsing.
    #[error("Invalid token value: expected {expected}, found {found}")]
    InvalidTokenValue {
        /// The expected token value type.
        expected: String,

        /// The found token value type.
        found: String,
    },
}
