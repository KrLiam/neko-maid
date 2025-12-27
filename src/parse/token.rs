//! Converts a NekoMaid UI file into a stream of tokens for parsing.

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use bevy::color::Color;

/// A token with its type and position.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The type of the token.
    pub token_type: TokenType,

    /// The position of the token in the input.
    pub position: TokenPosition,

    /// The raw value of the token.
    pub value: TokenValue,
}

/// The value stored within a token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    /// Used for tokens that do not carry a specific value, such as keywords or
    /// symbols.
    None,

    /// A string literal. (Also used for identifiers)
    String(String),

    /// A numeric literal.
    Number(f64),

    /// A color literal.
    Color(Color),

    /// A boolean literal.
    Boolean(bool),
}

impl TokenValue {
    /// Returns the name of the token value type.
    pub fn type_name(&self) -> &'static str {
        match self {
            TokenValue::None => "none",
            TokenValue::String(_) => "string",
            TokenValue::Number(_) => "number",
            TokenValue::Color(_) => "color",
            TokenValue::Boolean(_) => "boolean",
        }
    }

    /// Attempts to extract the numeric value from the token.
    pub fn as_number(self) -> Option<f64> {
        match self {
            TokenValue::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Attempts to extract the string value from the token.
    pub fn as_string(self) -> Option<String> {
        match self {
            TokenValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Attempts to extract the boolean value from the token.
    pub fn as_boolean(self) -> Option<bool> {
        match self {
            TokenValue::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Attempts to extract the color value from the token.
    pub fn as_color(self) -> Option<Color> {
        match self {
            TokenValue::Color(c) => Some(c),
            _ => None,
        }
    }
}

/// A token representing a lexical unit in the NekoMaid UI file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// An identifier token.
    Identifier,

    /// A string literal.
    StringLiteral,

    /// A numeric literal.
    NumberLiteral,

    /// A color literal.
    ColorLiteral,

    /// A boolean literal.
    BooleanLiteral,

    /// The with-class token.
    WithClass,

    /// The without-class token.
    WithoutClass,

    /// The Property Value token.
    PropertyValue,

    /// The End of Statement token.
    EndOfStatement,

    /// The Begin Properties token.
    BeginProperties,

    /// The End Properties token.
    EndProperties,

    /// The Percentage token.
    Percent,

    /// The Variable token.
    Variable,

    /// The `import` keyword.
    ImportKeyword,

    /// The `px` keyword.
    PxKeyword,

    /// The style keyword,
    StyleKeyword,

    /// The var keyword.
    VarKeyword,

    /// The layout keyword.
    LayoutKeyword,

    /// The with keyword.
    WithKeyword,
}

impl TokenType {
    /// Returns the name of the token type.
    pub fn type_name(&self) -> &'static str {
        match self {
            TokenType::Identifier => "identifier",
            TokenType::StringLiteral => "string",
            TokenType::NumberLiteral => "number",
            TokenType::ColorLiteral => "color",
            TokenType::BooleanLiteral => "boolean",
            TokenType::WithClass => "'+'",
            TokenType::WithoutClass => "'!'",
            TokenType::PropertyValue => "':'",
            TokenType::EndOfStatement => "';'",
            TokenType::BeginProperties => "'{'",
            TokenType::EndProperties => "'}'",
            TokenType::Percent => "'%'",
            TokenType::Variable => "'$'",
            TokenType::ImportKeyword => "'import'",
            TokenType::PxKeyword => "'px'",
            TokenType::StyleKeyword => "'style'",
            TokenType::VarKeyword => "'var'",
            TokenType::LayoutKeyword => "'layout'",
            TokenType::WithKeyword => "'with'",
        }
    }
}

/// The tokenizer struct that holds the position of the tokenizer.
struct TokenizerPosition<'a> {
    /// The input characters.
    chars: Peekable<Chars<'a>>,

    /// The current line number.
    line: usize,

    /// The current column number.
    column: usize,
}

/// Tokenizes the input string into a vector of tokens.
pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();

    let mut position = TokenizerPosition {
        chars: input.chars().peekable(),
        line: 1,
        column: 1,
    };

    while let Some(mut token) = next(&mut position)? {
        map_imports(&mut token);
        tokens.push(token);
    }

    Ok(tokens)
}

/// Maps certain identifier tokens to their keyword token types if necessary.
/// This includes mapping "import" to ImportKeyword and "px" to PxKeyword.
fn map_imports(token: &mut Token) {
    if token.token_type != TokenType::Identifier {
        return;
    }

    let TokenValue::String(ident) = &token.value else {
        return;
    };

    match ident.as_str() {
        "import" => {
            token.token_type = TokenType::ImportKeyword;
            token.value = TokenValue::None;
        }
        "px" => {
            token.token_type = TokenType::PxKeyword;
            token.value = TokenValue::None;
        }
        "true" => {
            token.token_type = TokenType::BooleanLiteral;
            token.value = TokenValue::Boolean(true);
        }
        "false" => {
            token.token_type = TokenType::BooleanLiteral;
            token.value = TokenValue::Boolean(false);
        }
        "style" => {
            token.token_type = TokenType::StyleKeyword;
            token.value = TokenValue::None;
        }
        "var" => {
            token.token_type = TokenType::VarKeyword;
            token.value = TokenValue::None;
        }
        "layout" => {
            token.token_type = TokenType::LayoutKeyword;
            token.value = TokenValue::None;
        }
        "with" => {
            token.token_type = TokenType::WithKeyword;
            token.value = TokenValue::None;
        }
        _ => {}
    }
}

/// Retrieves the next token from the tokenizer, if available.
fn next(position: &mut TokenizerPosition) -> Result<Option<Token>, TokenizeError> {
    loop {
        let c = match position.chars.peek().copied() {
            Some(ch) => ch,
            None => return Ok(None),
        };

        if c.is_whitespace() {
            position.chars.next();
            if c == '\n' {
                position.line += 1;
                position.column = 1;
            } else {
                position.column += 1;
            }
            continue;
        }

        match c {
            'a' ..= 'z' | 'A' ..= 'Z' | '_' => {
                let mut buffer = String::new();
                while let Some(c) = position.chars.next_if(identifier_char) {
                    buffer.push(c);
                    position.column += 1;
                }

                let len = buffer.len();
                return Ok(Some(Token {
                    token_type: TokenType::Identifier,
                    value: TokenValue::String(buffer),
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - len,
                        length: len,
                    },
                }));
            }
            '"' | '\'' | '`' => {
                let start = position.column;
                position.chars.next();
                position.column += 1;

                let mut buffer = String::new();
                for n in position.chars.by_ref() {
                    position.column += 1;
                    if n == c {
                        let len = buffer.len();
                        return Ok(Some(Token {
                            token_type: TokenType::StringLiteral,
                            value: TokenValue::String(buffer),
                            position: TokenPosition {
                                line: position.line,
                                column: start,
                                length: len + 2,
                            },
                        }));
                    } else {
                        buffer.push(n);
                    }
                }
                return Err(TokenizeError::UnexpectedEndOfInput);
            }
            '0' ..= '9' | '.' | '-' => {
                let mut buffer = String::new();
                while let Some(c) = position.chars.next_if(digit_char) {
                    buffer.push(c);
                }
                let len = buffer.len();
                let number = str_to_num(buffer, position)?;

                let start = position.column;
                position.column += len;
                return Ok(Some(Token {
                    token_type: TokenType::NumberLiteral,
                    value: TokenValue::Number(number),
                    position: TokenPosition {
                        line: position.line,
                        column: start,
                        length: len,
                    },
                }));
            }
            '$' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::Variable,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            '+' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::WithClass,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            '!' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::WithoutClass,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            '#' => {
                position.chars.next();

                let mut buffer = String::new();
                while let Some(c) = position.chars.next_if(hex_char) {
                    buffer.push(c);
                }

                let len = buffer.len() + 1;
                let color = hex_to_color(
                    &buffer,
                    TokenPosition {
                        line: position.line,
                        column: position.column,
                        length: len,
                    },
                )?;

                position.column += len;
                return Ok(Some(Token {
                    token_type: TokenType::ColorLiteral,
                    value: TokenValue::Color(color),
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - len,
                        length: len,
                    },
                }));
            }
            '{' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::BeginProperties,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            '}' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::EndProperties,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            ':' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::PropertyValue,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            ';' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::EndOfStatement,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            '%' => {
                position.chars.next();
                position.column += 1;
                return Ok(Some(Token {
                    token_type: TokenType::Percent,
                    value: TokenValue::None,
                    position: TokenPosition {
                        line: position.line,
                        column: position.column - 1,
                        length: 1,
                    },
                }));
            }
            '/' => {
                for c in position.chars.by_ref() {
                    position.column += 1;
                    if c == '\n' {
                        position.line += 1;
                        position.column = 1;
                        break;
                    }
                }
            }
            _ => {
                return Err(TokenizeError::UnexpectedCharacter(
                    c,
                    TokenPosition {
                        line: position.line,
                        column: position.column,
                        length: 1,
                    },
                ));
            }
        }
    }
}

/// Checks if a character is valid for an identifier.
fn identifier_char(c: &char) -> bool {
    c.is_ascii_alphanumeric() || *c == '_' || *c == '-'
}

/// Checks if a character is valid for a digit (including decimal point).
fn digit_char(c: &char) -> bool {
    c.is_ascii_digit() || *c == '.'
}

/// Checks if a character is a hexadecimal digit.
fn hex_char(c: &char) -> bool {
    c.is_ascii_hexdigit()
}

/// Converts a string to a number, returning an error if the format is invalid.
fn str_to_num(value: String, pos: &TokenizerPosition) -> Result<f64, TokenizeError> {
    let len = value.len();
    value.parse().map_err(|_| {
        TokenizeError::InvalidNumberFormat(
            value,
            TokenPosition {
                line: pos.line,
                column: pos.column,
                length: len,
            },
        )
    })
}

/// An error that occurs during tokenization.
#[derive(Debug, thiserror::Error)]
pub enum TokenizeError {
    /// An error that occurs due to an unexpected character.
    #[error("Unexpected character '{0}' at {1}")]
    UnexpectedCharacter(char, TokenPosition),

    /// An error that occurs due to unexpected end of input.
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    /// An error that occurs due to invalid number format.
    #[error("Invalid number format: '{0}' at {1}")]
    InvalidNumberFormat(String, TokenPosition),

    /// An error that occurs due to invalid color format.
    #[error("Invalid color format: '{0}' at {1}")]
    InvalidColorFormat(String, TokenPosition),
}

/// Represents the position of a token within the input string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenPosition {
    /// The line number of the token.
    pub line: usize,

    /// The column number of the token.
    pub column: usize,

    /// The length of the token span.
    pub length: usize,
}

impl Default for TokenPosition {
    fn default() -> Self {
        TokenPosition {
            line: 1,
            column: 1,
            length: 0,
        }
    }
}

impl fmt::Display for TokenPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "line {}, col {}-{}",
            self.line,
            self.column,
            self.column + self.length - 1
        )
    }
}

/// Converts a hexadecimal string to a Color.
///
/// Supports the following formats:
/// - RGB (e.g., "FFF")
/// - RRGGBB (e.g., "FFFFFF")
/// - RRGGBBAA (e.g., "FFFFFFFF")
fn hex_to_color(hex: &str, pos: TokenPosition) -> Result<Color, TokenizeError> {
    match hex.len() {
        3 => {
            let r = hex_to_byte(&hex[0 .. 1].repeat(2))?;
            let g = hex_to_byte(&hex[1 .. 2].repeat(2))?;
            let b = hex_to_byte(&hex[2 .. 3].repeat(2))?;
            Ok(Color::srgb_u8(r, g, b))
        }
        6 => {
            let r = hex_to_byte(&hex[0 .. 2])?;
            let g = hex_to_byte(&hex[2 .. 4])?;
            let b = hex_to_byte(&hex[4 .. 6])?;
            Ok(Color::srgb_u8(r, g, b))
        }
        8 => {
            let r = hex_to_byte(&hex[0 .. 2])?;
            let g = hex_to_byte(&hex[2 .. 4])?;
            let b = hex_to_byte(&hex[4 .. 6])?;
            let a = hex_to_byte(&hex[6 .. 8])?;
            Ok(Color::srgba_u8(r, g, b, a))
        }
        _ => Err(TokenizeError::InvalidColorFormat(hex.to_string(), pos)),
    }
}

/// Converts a hexadecimal string to a byte. Returns an error if the format is
/// invalid, or cannot be stored within a byte.
fn hex_to_byte(str: &str) -> Result<u8, TokenizeError> {
    u8::from_str_radix(str, 16).map_err(|_| {
        TokenizeError::InvalidColorFormat(
            str.to_string(),
            TokenPosition {
                line: 0,
                column: 0,
                length: str.len(),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    fn pos(line: usize, col: usize, length: usize) -> TokenPosition {
        TokenPosition {
            line,
            column: col,
            length,
        }
    }

    fn ident(value: &str, pos: TokenPosition) -> Token {
        Token {
            token_type: TokenType::Identifier,
            value: TokenValue::String(value.to_string()),
            position: pos,
        }
    }

    fn str(value: &str, pos: TokenPosition) -> Token {
        Token {
            token_type: TokenType::StringLiteral,
            value: TokenValue::String(value.to_string()),
            position: pos,
        }
    }

    fn num(value: f64, pos: TokenPosition) -> Token {
        Token {
            token_type: TokenType::NumberLiteral,
            value: TokenValue::Number(value),
            position: pos,
        }
    }

    fn color(value: &str, pos: TokenPosition) -> Token {
        Token {
            token_type: TokenType::ColorLiteral,
            value: TokenValue::Color(hex_to_color(value, pos).unwrap()),
            position: pos,
        }
    }

    fn token(t: TokenType, pos: TokenPosition) -> Token {
        Token {
            token_type: t,
            value: TokenValue::None,
            position: pos,
        }
    }

    #[test]
    fn test_color() {
        let c1 = hex_to_color("FFF", pos(1, 1, 4)).unwrap();
        assert_eq!(c1, Color::srgb_u8(255, 255, 255));

        let c2 = hex_to_color("000", pos(1, 1, 4)).unwrap();
        assert_eq!(c2, Color::srgb_u8(0, 0, 0));

        let c3 = hex_to_color("FF5733", pos(1, 1, 7)).unwrap();
        assert_eq!(c3, Color::srgb_u8(255, 87, 51));

        let c4 = hex_to_color("0F5733", pos(1, 1, 7)).unwrap();
        assert_eq!(c4, Color::srgb_u8(15, 87, 51));

        let c5 = hex_to_color("FF5733CC", pos(1, 1, 9)).unwrap();
        assert_eq!(c5, Color::srgba_u8(255, 87, 51, 204));

        let c6 = hex_to_color("0F573380", pos(1, 1, 9)).unwrap();
        assert_eq!(c6, Color::srgba_u8(15, 87, 51, 128));

        let c7 = hex_to_color("abc", pos(1, 1, 4)).unwrap();
        assert_eq!(c7, Color::srgb_u8(170, 187, 204));
    }

    #[test]
    fn test_invalid_colors() {
        assert!(hex_to_color("GGG", pos(1, 1, 4)).is_err());
        assert!(hex_to_color("FFFF", pos(1, 1, 5)).is_err());
        assert!(hex_to_color("ZZZZZZ", pos(1, 1, 7)).is_err());
        assert!(hex_to_color("12345", pos(1, 1, 6)).is_err());
        assert!(hex_to_color("a", pos(1, 1, 2)).is_err());
    }

    #[test]
    fn test_tokenize_identifier() {
        let x1 = tokenize("  my_identifier  ").unwrap();
        let y1 = ident("my_identifier", pos(1, 3, 13));
        assert_eq!(x1, vec![y1]);

        let x2 = tokenize("  anotherIdentifier123  ").unwrap();
        let y2 = ident("anotherIdentifier123", pos(1, 3, 20));
        assert_eq!(x2, vec![y2]);

        let x3 = tokenize("  _leadingUnderscore  ").unwrap();
        let y3 = ident("_leadingUnderscore", pos(1, 3, 18));
        assert_eq!(x3, vec![y3]);

        let x4 = tokenize("dash-separated-identifier").unwrap();
        let y4 = ident("dash-separated-identifier", pos(1, 1, 25));
        assert_eq!(x4, vec![y4]);
    }

    #[test]
    fn test_tokenize_string_literal() {
        let x1 = tokenize("  \"Hello, World!\"  ").unwrap();
        let y1 = str("Hello, World!", pos(1, 3, 15));
        assert_eq!(x1, vec![y1]);

        let x2 = tokenize("  'Single quoted string'  ").unwrap();
        let y2 = str("Single quoted string", pos(1, 3, 22));
        assert_eq!(x2, vec![y2]);

        let x3 = tokenize("  `Backtick string`  ").unwrap();
        let y3 = str("Backtick string", pos(1, 3, 17));
        assert_eq!(x3, vec![y3]);

        let x4 = tokenize("  \"String with spaces and 123!@#\"  ").unwrap();
        let y4 = str("String with spaces and 123!@#", pos(1, 3, 31));
        assert_eq!(x4, vec![y4]);

        let x5 = tokenize("  \"With another `string` inside\"  ").unwrap();
        let y5 = str("With another `string` inside", pos(1, 3, 30));
        assert_eq!(x5, vec![y5]);
    }

    #[test]
    fn test_tokenize_number_literal() {
        let x1 = tokenize("\n  42 \t ").unwrap();
        let y1 = num(42f64, pos(2, 3, 2));
        assert_eq!(x1, vec![y1]);

        let x2 = tokenize("\n\t 3.2 ").unwrap();
        let y2 = num(3.2f64, pos(2, 3, 3));
        assert_eq!(x2, vec![y2]);

        let x3 = tokenize("  0.  ").unwrap();
        let y3 = num(0f64, pos(1, 3, 2));
        assert_eq!(x3, vec![y3]);

        let x4 = tokenize("  .75  ").unwrap();
        let y4 = num(0.75f64, pos(1, 3, 3));
        assert_eq!(x4, vec![y4]);
    }

    #[test]
    fn test_tokenize_mixed_tokens() {
        let input = r#"
import "styles.neko_ui";

button {
    width: 100%;
    height: 50px;
    background-color: #0F5733;
}
"#;

        let tokens = tokenize(input).unwrap();
        let expected_tokens = vec![
            token(TokenType::ImportKeyword, pos(2, 1, 6)),
            str("styles.neko_ui", pos(2, 8, 16)),
            token(TokenType::EndOfStatement, pos(2, 24, 1)),
            ident("button", pos(4, 1, 6)),
            token(TokenType::BeginProperties, pos(4, 8, 1)),
            ident("width", pos(5, 5, 5)),
            token(TokenType::PropertyValue, pos(5, 10, 1)),
            num(100f64, pos(5, 12, 3)),
            token(TokenType::Percent, pos(5, 15, 1)),
            token(TokenType::EndOfStatement, pos(5, 16, 1)),
            ident("height", pos(6, 5, 6)),
            token(TokenType::PropertyValue, pos(6, 11, 1)),
            num(50f64, pos(6, 13, 2)),
            token(TokenType::PxKeyword, pos(6, 15, 2)),
            token(TokenType::EndOfStatement, pos(6, 17, 1)),
            ident("background-color", pos(7, 5, 16)),
            token(TokenType::PropertyValue, pos(7, 21, 1)),
            color("0F5733", pos(7, 23, 7)),
            token(TokenType::EndOfStatement, pos(7, 30, 1)),
            token(TokenType::EndProperties, pos(8, 1, 1)),
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
