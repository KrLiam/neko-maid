//! This module defines the data nodes for NekoMaid UI files.

use bevy::color::Color;

use crate::parse::token::TokenPosition;

/// A top-level node representing a NekoMaid UI file.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ModuleNode {
    /// A list of import statements in the file.
    pub imports: Vec<ImportNode>,

    /// A list of variable declarations in the file.
    pub variables: Vec<PropertyNode>,

    /// A list of style elements in the file.
    pub styles: Vec<StyleNode>,

    /// A list of layout elements in the file.
    pub layouts: Vec<LayoutNode>,
}

/// A node representing an import statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ImportNode {
    /// The path of the imported file.
    pub path: String,

    /// The position of the import statement in the source file. (In case of
    /// error reporting)
    pub position: TokenPosition,
}

/// A node representing an element.
#[derive(Debug, Clone, PartialEq)]
pub struct StyleNode {
    /// The selector for the style.
    pub selector: SelectorNode,

    /// A list of property nodes associated with the element.
    pub properties: Vec<PropertyNode>,

    /// A list of child element nodes.
    pub children: Vec<StyleNode>,
}

/// A node representing an element.
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutNode {
    /// The name of the widget.
    pub widget: String,

    /// A list of classes associated with the element.
    pub classes: Vec<String>,

    /// A list of property nodes associated with the element.
    pub properties: Vec<PropertyNode>,

    /// A list of child element nodes.
    pub children: Vec<LayoutNode>,

    /// The position of the element in the source file. (In case of error
    /// reporting)
    ///
    /// This reports the position of the widget name token.
    pub position: TokenPosition,
}

/// A node representing a property within a style or element.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyNode {
    /// The name of the property.
    pub name: String,

    /// The value of the property.
    pub value: PropertyNodeValue,

    /// The position of the property in the source file. (In case of error
    /// reporting)
    pub position: TokenPosition,
}

/// A node representing a property value.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyNodeValue {
    /// A string value.
    String(String),

    /// A numeric value.
    Number(f64),

    /// A pixel value.
    Pixels(f64),

    /// A percentage value.
    Percent(f64),

    /// A boolean value.
    Bool(bool),

    /// A color value.
    Color(Color),

    /// A reference to a variable.
    Variable {
        /// The name of the variable.
        name: String,

        /// The position of the variable token. (In case of error reporting)
        position: TokenPosition,
    },
}

/// A node representing a selector in a style definition.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectorNode {
    /// The widget name of the selector.
    pub widget: String,

    /// The parts of the selector.
    pub parts: Vec<SelectorPart>,

    /// The position of the selector in the source file. (In case of error
    /// reporting)
    ///
    /// This reports the position of the widget name token.
    pub position: TokenPosition,
}

/// A part of a selector.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPart {
    /// A class selector.
    WithClass(String),

    /// A class exclusion selector.
    WithoutClass(String),
}
