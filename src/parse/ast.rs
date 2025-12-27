//! Defines the AST (Abstract Syntax Tree) structures used in the NekoMaid UI
//! file parser.

use std::iter::Peekable;
use std::vec::IntoIter;

use bevy::color::Color;

use super::nodes::*;
use super::token::Token;
use crate::parse::NekoMaidParseError;
use crate::parse::token::{TokenType, TokenValue};

/// Type alias for a peekable iterator over tokens.
type Tokens = Peekable<IntoIter<Token>>;

/// Builds an AST from a list of tokens.
pub fn build_ast(tokens: Vec<Token>) -> Result<ModuleNode, NekoMaidParseError> {
    let mut tokens = tokens.into_iter().peekable();
    parse_root(&mut tokens)
}

/// Parses the root of the AST.
fn parse_root(tokens: &mut Tokens) -> Result<ModuleNode, NekoMaidParseError> {
    let mut file_node = ModuleNode::default();

    while let Some(next) = tokens.peek() {
        match next.token_type {
            TokenType::ImportKeyword => {
                let import_node = parse_import(tokens)?;
                file_node.imports.push(import_node);
            }
            TokenType::VarKeyword => {
                let variable = parse_variable(tokens)?;
                file_node.variables.push(variable);
            }
            TokenType::StyleKeyword => {
                let style_node = parse_style(tokens)?;
                file_node.styles.push(style_node);
            }
            TokenType::LayoutKeyword => {
                let layout_node = parse_layout(tokens)?;
                file_node.layouts.push(layout_node);
            }
            _ => {
                return Err(NekoMaidParseError::UnexpectedToken {
                    expected: vec![
                        TokenType::ImportKeyword.type_name().to_string(),
                        TokenType::VarKeyword.type_name().to_string(),
                        TokenType::StyleKeyword.type_name().to_string(),
                        TokenType::LayoutKeyword.type_name().to_string(),
                    ],
                    found: next.token_type.type_name().to_string(),
                    position: next.position,
                });
            }
        }
    }

    Ok(file_node)
}

/// Parses an import statement.
fn parse_import(tokens: &mut Tokens) -> Result<ImportNode, NekoMaidParseError> {
    expect(tokens, TokenType::ImportKeyword)?;
    let position = tokens.peek().map(|t| t.position).unwrap_or_default();
    let path = expect(tokens, TokenType::StringLiteral)?;
    let path = as_string(path)?;
    expect(tokens, TokenType::EndOfStatement)?;
    Ok(ImportNode { path, position })
}

/// Parses a variable declaration.
fn parse_variable(tokens: &mut Tokens) -> Result<PropertyNode, NekoMaidParseError> {
    expect(tokens, TokenType::VarKeyword)?;
    let property = parse_property(tokens)?;
    Ok(property)
}

/// Parses a property statement.
fn parse_property(tokens: &mut Tokens) -> Result<PropertyNode, NekoMaidParseError> {
    let name = expect(tokens, TokenType::Identifier)?;
    let name = as_string(name)?;
    expect(tokens, TokenType::PropertyValue)?;
    let position = tokens.peek().map(|t| t.position).unwrap_or_default();
    let value = parse_value(tokens)?;
    expect(tokens, TokenType::EndOfStatement)?;

    Ok(PropertyNode {
        name,
        value,
        position,
    })
}

/// Parses a property value.
///
/// (Does not check for the end of statement; that is handled by the caller.)
fn parse_value(tokens: &mut Tokens) -> Result<PropertyNodeValue, NekoMaidParseError> {
    let value = next(tokens)?;

    match value.token_type {
        TokenType::StringLiteral | TokenType::Identifier => {
            let val = as_string(value.value)?;
            Ok(PropertyNodeValue::String(val))
        }
        TokenType::NumberLiteral => {
            let val = as_number(value.value)?;

            if maybe_next(tokens, TokenType::PxKeyword).is_some() {
                return Ok(PropertyNodeValue::Pixels(val));
            }

            if maybe_next(tokens, TokenType::Percent).is_some() {
                return Ok(PropertyNodeValue::Percent(val));
            }

            Ok(PropertyNodeValue::Number(val))
        }
        TokenType::BooleanLiteral => {
            let val = as_boolean(value.value)?;
            Ok(PropertyNodeValue::Bool(val))
        }
        TokenType::ColorLiteral => {
            let val = as_color(value.value)?;
            Ok(PropertyNodeValue::Color(val))
        }
        TokenType::Variable => {
            let var_name = as_string(expect(tokens, TokenType::Identifier)?)?;
            Ok(PropertyNodeValue::Variable {
                name: var_name,
                position: value.position,
            })
        }
        _ => Err(NekoMaidParseError::UnexpectedToken {
            expected: vec![
                TokenType::StringLiteral.type_name().to_string(),
                TokenType::NumberLiteral.type_name().to_string(),
                TokenType::BooleanLiteral.type_name().to_string(),
                TokenType::ColorLiteral.type_name().to_string(),
                TokenType::Variable.type_name().to_string(),
            ],
            found: value.token_type.type_name().to_string(),
            position: value.position,
        }),
    }
}

/// Parses a style.
fn parse_style(tokens: &mut Tokens) -> Result<StyleNode, NekoMaidParseError> {
    expect(tokens, TokenType::StyleKeyword)?;
    let block = parse_style_block(tokens)?;
    Ok(block)
}

/// Parses a style block.
fn parse_style_block(tokens: &mut Tokens) -> Result<StyleNode, NekoMaidParseError> {
    let selector = parse_selector(tokens)?;
    let mut node = StyleNode {
        selector,
        properties: Vec::new(),
        children: Vec::new(),
    };

    expect(tokens, TokenType::BeginProperties)?;

    while let Some(next) = tokens.peek() {
        match next.token_type {
            TokenType::EndProperties => break,
            TokenType::Identifier => {
                let property = parse_property(tokens)?;
                node.properties.push(property);
            }
            TokenType::WithKeyword => {
                expect(tokens, TokenType::WithKeyword)?;
                let child_style = parse_style_block(tokens)?;
                node.children.push(child_style);
            }
            _ => {
                return Err(NekoMaidParseError::UnexpectedToken {
                    expected: vec![
                        TokenType::Identifier.type_name().to_string(),
                        TokenType::WithKeyword.type_name().to_string(),
                        TokenType::EndProperties.type_name().to_string(),
                    ],
                    found: next.token_type.type_name().to_string(),
                    position: next.position,
                });
            }
        }
    }

    expect(tokens, TokenType::EndProperties)?;

    Ok(node)
}

/// Parses a style selector expression.
fn parse_selector(tokens: &mut Tokens) -> Result<SelectorNode, NekoMaidParseError> {
    let position = tokens.peek().map(|t| t.position).unwrap_or_default();
    let widget = as_string(expect(tokens, TokenType::Identifier)?)?;

    let mut selector = SelectorNode {
        widget,
        parts: Vec::new(),
        position,
    };

    while let Some(next) = tokens.peek() {
        match next.token_type {
            TokenType::WithClass => {
                expect(tokens, TokenType::WithClass)?;
                let class = as_string(expect(tokens, TokenType::Identifier)?)?;
                selector.parts.push(SelectorPart::WithClass(class));
            }
            TokenType::WithoutClass => {
                expect(tokens, TokenType::WithoutClass)?;
                let class = as_string(expect(tokens, TokenType::Identifier)?)?;
                selector.parts.push(SelectorPart::WithoutClass(class));
            }
            TokenType::BeginProperties => break,
            other => {
                return Err(NekoMaidParseError::UnexpectedToken {
                    expected: vec![
                        TokenType::WithClass.type_name().to_string(),
                        TokenType::WithoutClass.type_name().to_string(),
                        TokenType::BeginProperties.type_name().to_string(),
                    ],
                    found: other.type_name().to_string(),
                    position: next.position,
                });
            }
        }
    }

    Ok(selector)
}

/// Parses a layout.
fn parse_layout(tokens: &mut Tokens) -> Result<LayoutNode, NekoMaidParseError> {
    expect(tokens, TokenType::LayoutKeyword)?;
    let layout = parse_layout_block(tokens)?;
    Ok(layout)
}

/// Parses a layout block.
fn parse_layout_block(tokens: &mut Tokens) -> Result<LayoutNode, NekoMaidParseError> {
    let position = tokens.peek().map(|t| t.position).unwrap_or_default();
    let widget = as_string(expect(tokens, TokenType::Identifier)?)?;
    expect(tokens, TokenType::BeginProperties)?;

    let mut layout = LayoutNode {
        widget,
        classes: Vec::new(),
        properties: Vec::new(),
        children: Vec::new(),
        position,
    };

    while let Some(next) = tokens.peek() {
        match next.token_type {
            TokenType::EndProperties => break,
            TokenType::Identifier => {
                let property = parse_property(tokens)?;
                layout.properties.push(property);
            }
            TokenType::WithKeyword => {
                expect(tokens, TokenType::WithKeyword)?;
                let child_layout = parse_layout_block(tokens)?;
                layout.children.push(child_layout);
            }
            TokenType::WithClass => {
                expect(tokens, TokenType::WithClass)?;
                let class = as_string(expect(tokens, TokenType::Identifier)?)?;
                expect(tokens, TokenType::EndOfStatement)?;
                layout.classes.push(class);
            }
            _ => {
                return Err(NekoMaidParseError::UnexpectedToken {
                    expected: vec![
                        TokenType::Identifier.type_name().to_string(),
                        TokenType::WithKeyword.type_name().to_string(),
                        TokenType::WithClass.type_name().to_string(),
                        TokenType::EndProperties.type_name().to_string(),
                    ],
                    found: next.token_type.type_name().to_string(),
                    position: next.position,
                });
            }
        }
    }

    expect(tokens, TokenType::EndProperties)?;
    Ok(layout)
}

/// Retrieves the next token from the token iterator, or returns an error if the
/// end of the stream is reached.
fn next(tokens: &mut Tokens) -> Result<Token, NekoMaidParseError> {
    tokens.next().ok_or(NekoMaidParseError::EndOfStream)
}

/// Checks if the next token exists and is of the specified type. If so,
/// advances the iterator and returns its value; otherwise, returns None.
fn maybe_next(tokens: &mut Tokens, ty: TokenType) -> Option<TokenValue> {
    let next = tokens.peek()?;
    if next.token_type == ty {
        Some(tokens.next().unwrap().value)
    } else {
        None
    }
}

/// Expects the next token to be of the specified type. Returns the value stored
/// within the token if the expectation is met; otherwise, returns an error.
fn expect(tokens: &mut Tokens, expected: TokenType) -> Result<TokenValue, NekoMaidParseError> {
    let next = next(tokens)?;

    if next.token_type == expected {
        Ok(next.value)
    } else {
        Err(NekoMaidParseError::UnexpectedToken {
            expected: vec![expected.type_name().to_string()],
            found: next.token_type.type_name().to_string(),
            position: next.position,
        })
    }
}

/// Expects the next token to be a string literal and returns its value.
fn as_string(value: TokenValue) -> Result<String, NekoMaidParseError> {
    match value {
        TokenValue::String(s) => Ok(s),
        _ => Err(NekoMaidParseError::InvalidTokenValue {
            expected: "string".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

/// Expects the next token to be a number literal and returns its value.
fn as_number(value: TokenValue) -> Result<f64, NekoMaidParseError> {
    match value {
        TokenValue::Number(n) => Ok(n),
        _ => Err(NekoMaidParseError::InvalidTokenValue {
            expected: "number".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

/// Expects the next token to be a boolean literal and returns its value.
fn as_boolean(value: TokenValue) -> Result<bool, NekoMaidParseError> {
    match value {
        TokenValue::Boolean(b) => Ok(b),
        _ => Err(NekoMaidParseError::InvalidTokenValue {
            expected: "boolean".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

/// Expects the next token to be a color literal and returns its value.
fn as_color(value: TokenValue) -> Result<Color, NekoMaidParseError> {
    match value {
        TokenValue::Color(c) => Ok(c),
        _ => Err(NekoMaidParseError::InvalidTokenValue {
            expected: "color".to_string(),
            found: value.type_name().to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::parse::parse_neko_ui;
    use crate::parse::token::TokenPosition;

    #[test]
    fn imports() {
        const SOURCE: &str = r#"
import "path/to/module1";
import "module2";
        "#;

        let module = ModuleNode {
            imports: vec![
                ImportNode {
                    path: "path/to/module1".to_string(),
                    position: TokenPosition {
                        line: 2,
                        column: 8,
                        length: 17,
                    },
                },
                ImportNode {
                    path: "module2".to_string(),
                    position: TokenPosition {
                        line: 3,
                        column: 8,
                        length: 9,
                    },
                },
            ],
            variables: Vec::new(),
            styles: Vec::new(),
            layouts: Vec::new(),
        };

        let src = parse_neko_ui(SOURCE).unwrap();
        assert_eq!(src, module);
    }

    #[test]
    fn variables() {
        const SOURCE: &str = r#"
var primary-color: #ff0000;
var is-active: true;
var padding: 10px;
        "#;

        let module = ModuleNode {
            imports: Vec::new(),
            variables: vec![
                PropertyNode {
                    name: "primary-color".to_string(),
                    value: PropertyNodeValue::Color(Color::srgb(1.0, 0.0, 0.0)),
                    position: TokenPosition {
                        line: 2,
                        column: 20,
                        length: 7,
                    },
                },
                PropertyNode {
                    name: "is-active".to_string(),
                    value: PropertyNodeValue::Bool(true),
                    position: TokenPosition {
                        line: 3,
                        column: 16,
                        length: 4,
                    },
                },
                PropertyNode {
                    name: "padding".to_string(),
                    value: PropertyNodeValue::Pixels(10.0),
                    position: TokenPosition {
                        line: 4,
                        column: 14,
                        length: 2,
                    },
                },
            ],
            styles: Vec::new(),
            layouts: Vec::new(),
        };

        let src = parse_neko_ui(SOURCE).unwrap();
        assert_eq!(src, module);
    }

    #[test]
    fn style() {
        const SOURCE: &str = r#"
style div +hovered !pressed {
    width: 100px;

    with p {
        color: #00ff00;
    }
}
        "#;

        let module = ModuleNode {
            imports: Vec::new(),
            variables: Vec::new(),
            styles: vec![StyleNode {
                selector: SelectorNode {
                    widget: "div".to_string(),
                    parts: vec![
                        SelectorPart::WithClass("hovered".to_string()),
                        SelectorPart::WithoutClass("pressed".to_string()),
                    ],
                    position: TokenPosition {
                        line: 2,
                        column: 7,
                        length: 3,
                    },
                },
                properties: vec![PropertyNode {
                    name: "width".to_string(),
                    value: PropertyNodeValue::Pixels(100.0),
                    position: TokenPosition {
                        line: 3,
                        column: 12,
                        length: 3,
                    },
                }],
                children: vec![StyleNode {
                    selector: SelectorNode {
                        widget: "p".to_string(),
                        parts: Vec::new(),
                        position: TokenPosition {
                            line: 5,
                            column: 10,
                            length: 1,
                        },
                    },
                    properties: vec![PropertyNode {
                        name: "color".to_string(),
                        value: PropertyNodeValue::Color(Color::srgb(0.0, 1.0, 0.0)),
                        position: TokenPosition {
                            line: 6,
                            column: 16,
                            length: 7,
                        },
                    }],
                    children: Vec::new(),
                }],
            }],
            layouts: Vec::new(),
        };

        let src = parse_neko_ui(SOURCE).unwrap();
        assert_eq!(src, module)
    }

    #[test]
    fn layout() {
        const SOURCE: &str = r#"
layout div {
    +outer-menu;

    with button {
        border-color: #0000ff;
        border-width: 2px;
    }
}
        "#;

        let module = ModuleNode {
            imports: Vec::new(),
            variables: Vec::new(),
            styles: Vec::new(),
            layouts: vec![LayoutNode {
                widget: "div".to_string(),
                classes: vec!["outer-menu".to_string()],
                properties: Vec::new(),
                children: vec![LayoutNode {
                    widget: "button".to_string(),
                    classes: Vec::new(),
                    properties: vec![
                        PropertyNode {
                            name: "border-color".to_string(),
                            value: PropertyNodeValue::Color(Color::srgb(0.0, 0.0, 1.0)),
                            position: TokenPosition {
                                line: 6,
                                column: 23,
                                length: 7,
                            },
                        },
                        PropertyNode {
                            name: "border-width".to_string(),
                            value: PropertyNodeValue::Pixels(2.0),
                            position: TokenPosition {
                                line: 7,
                                column: 23,
                                length: 1,
                            },
                        },
                    ],
                    children: Vec::new(),
                    position: TokenPosition {
                        line: 5,
                        column: 10,
                        length: 6,
                    },
                }],
                position: TokenPosition {
                    line: 2,
                    column: 8,
                    length: 3,
                },
            }],
        };

        let src = parse_neko_ui(SOURCE).unwrap();
        assert_eq!(src, module)
    }
}
