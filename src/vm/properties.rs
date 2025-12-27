//! Defines the NekoMaid UI widget properties and their types.

use std::collections::HashMap;

use bevy::color::Color;

use crate::parse::nodes::PropertyNodeValue;
use crate::vm::NekoMaidVMError;
use crate::vm::allocator::{NekoContextAllocator, NekoProperty, NekoWidget};
use crate::vm::context::NekoContext;
use crate::vm::style::{NekoStyle, SelectorHierarchy};

/// Defines a NekoMaid UI widget.
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetDefinition {
    /// The widget being defined.
    pub(super) widget: NekoWidget,

    /// The properties of the widget.
    pub(super) properties: HashMap<NekoProperty, PropertyDefinition>,
}

impl WidgetDefinition {
    /// Creates a new [`NekoWidgetDefinition`] instance.
    pub fn new(widget: NekoWidget) -> Self {
        Self {
            widget,
            properties: HashMap::new(),
        }
    }

    /// Returns the [`Widget`] being defined.
    pub fn widget(&self) -> NekoWidget {
        self.widget
    }

    /// Creates a new property definition for this widget, overwriting any
    /// existing definition with the same name.
    pub fn add_property(&mut self, property: PropertyDefinition) {
        self.properties.insert(property.property, property);
    }

    /// Gets the property definition from this widget by its name.
    pub fn get_property(&self, property: NekoProperty) -> Option<&PropertyDefinition> {
        self.properties.get(&property)
    }

    /// Returns all property definitions of this widget.
    pub fn properties(&self) -> &HashMap<NekoProperty, PropertyDefinition> {
        &self.properties
    }

    /// Generates the default style for this widget, based on its property
    /// definitions.
    pub fn default_style(&self) -> NekoStyle {
        let mut style = NekoStyle::new(SelectorHierarchy::from(self.widget));
        for (property, definition) in &self.properties {
            style.set_property(*property, definition.default_value.clone());
        }
        style
    }
}

/// A property of a NekoMaid UI widget.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyDefinition {
    /// The [`Property`] being defined.
    pub(super) property: NekoProperty,

    /// The default value of the property.
    pub(super) default_value: PropertyValue,
}

impl PropertyDefinition {
    /// Creates a new [`WidgetProperty`] instance.
    pub fn new<V: Into<PropertyValue>>(property: NekoProperty, default_value: V) -> Self {
        Self {
            property,
            default_value: default_value.into(),
        }
    }

    /// Returns the [`Property`] being defined.
    pub fn property(&self) -> NekoProperty {
        self.property
    }

    /// Returns the default value of the property.
    pub fn default_value(&self) -> &PropertyValue {
        &self.default_value
    }
}

/// A value of a NekoMaid UI element property.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// A string value.
    String(String),

    /// A numeric value.
    Number(f64),

    /// A boolean value.
    Bool(bool),

    /// A color value.
    Color(Color),

    /// A percentage number value.
    Percent(f64),

    /// A pixel number value.
    Pixels(f64),
}

impl PropertyValue {
    /// Returns the type of this property value.
    pub fn value_type(&self) -> PropertyType {
        match self {
            PropertyValue::String(_) => PropertyType::String,
            PropertyValue::Number(_) => PropertyType::Number,
            PropertyValue::Bool(_) => PropertyType::Boolean,
            PropertyValue::Color(_) => PropertyType::Color,
            PropertyValue::Percent(_) => PropertyType::Percentage,
            PropertyValue::Pixels(_) => PropertyType::Pixels,
        }
    }

    /// Converts a [`PropertyNodeValue`] into a [`PropertyValue`].
    pub fn from_property_node_value(
        value: PropertyNodeValue,
        ctx: &NekoContext,
    ) -> Result<PropertyValue, NekoMaidVMError> {
        match value {
            PropertyNodeValue::String(s) => Ok(PropertyValue::String(s)),
            PropertyNodeValue::Number(n) => Ok(PropertyValue::Number(n)),
            PropertyNodeValue::Pixels(p) => Ok(PropertyValue::Pixels(p)),
            PropertyNodeValue::Percent(p) => Ok(PropertyValue::Percent(p)),
            PropertyNodeValue::Bool(b) => Ok(PropertyValue::Bool(b)),
            PropertyNodeValue::Color(c) => Ok(PropertyValue::Color(c)),
            PropertyNodeValue::Variable { name, position } => ctx
                .get_variable(NekoContextAllocator::get_or_create_variable(&name))
                .cloned()
                .ok_or(NekoMaidVMError::VariableNotFound(name, position)),
        }
    }

    /// Returns a reference to this property value.
    pub fn as_ref(&self) -> PropertyValueRef<'_> {
        match self {
            PropertyValue::String(s) => PropertyValueRef::String(s),
            PropertyValue::Number(n) => PropertyValueRef::Number(*n),
            PropertyValue::Bool(b) => PropertyValueRef::Bool(*b),
            PropertyValue::Color(c) => PropertyValueRef::Color(*c),
            PropertyValue::Percent(p) => PropertyValueRef::Percent(*p),
            PropertyValue::Pixels(p) => PropertyValueRef::Pixels(*p),
        }
    }
}

impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        PropertyValue::String(value)
    }
}

impl From<&str> for PropertyValue {
    fn from(value: &str) -> Self {
        PropertyValue::String(value.to_string())
    }
}

impl From<f64> for PropertyValue {
    fn from(value: f64) -> Self {
        PropertyValue::Number(value)
    }
}

impl From<bool> for PropertyValue {
    fn from(value: bool) -> Self {
        PropertyValue::Bool(value)
    }
}

impl From<Color> for PropertyValue {
    fn from(value: Color) -> Self {
        PropertyValue::Color(value)
    }
}

/// A reference to a value of a NekoMaid UI element property.
///
/// This is a utility enum, intended to make using match statements easier by
/// inlining certain references. (Namely, strings.)
///
/// ```rust
/// use bevy::prelude::Val;
/// use neko_maid::vm::properties::{PropertyValue, PropertyValueRef};
///
/// fn property_to_val(value: &PropertyValue) -> Option<Val> {
///     match value.as_ref() {
///         PropertyValueRef::String("auto") => Some(Val::Auto),
///         PropertyValueRef::Pixels(n) => Some(Val::Px(n as f32)),
///         PropertyValueRef::Percent(n) => Some(Val::Percent(n as f32)),
///         other => None
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValueRef<'a> {
    /// A string value.
    String(&'a str),

    /// A numeric value.
    Number(f64),

    /// A boolean value.
    Bool(bool),

    /// A color value.
    Color(Color),

    /// A percentage number value.
    Percent(f64),

    /// A pixel number value.
    Pixels(f64),
}

/// The type of a widget property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyType {
    /// A string type.
    String,

    /// A numeric type.
    Number,

    /// A boolean type.
    Boolean,

    /// A color type.
    Color,

    /// A percentage type.
    Percentage,

    /// A pixel type.
    Pixels,
}
