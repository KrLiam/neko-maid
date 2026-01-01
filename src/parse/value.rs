//! A module for working with NekoMaid UI element property values.

use std::fmt;

use bevy::prelude::*;
use bevy::text::{FontSmoothing, LineHeight};

use crate::parse::property::PropertyType;

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
}

impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        PropertyValue::String(value)
    }
}

impl From<&String> for PropertyValue {
    fn from(value: &String) -> Self {
        PropertyValue::String(value.clone())
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

impl From<i32> for PropertyValue {
    fn from(value: i32) -> Self {
        PropertyValue::Number(value as f64)
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

impl fmt::Display for PropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyValue::String(s) => write!(f, "\"{}\"", s),
            PropertyValue::Number(n) => write!(f, "{}", n),
            PropertyValue::Bool(b) => write!(f, "{}", b),
            PropertyValue::Percent(p) => write!(f, "{}%", p),
            PropertyValue::Pixels(px) => write!(f, "{}px", px),
            PropertyValue::Color(c) => write!(f, "{}", c.to_srgba().to_hex()),
        }
    }
}

impl From<&PropertyValue> for Val {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "auto" => Val::Auto,
            PropertyValue::Pixels(n) => Val::Px(*n as f32),
            PropertyValue::Percent(n) => Val::Percent(*n as f32),
            PropertyValue::Number(n) => Val::Px(*n as f32),
            _ => {
                warn_once!("Failed to convert PropertyValue {} to Val", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for Color {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::Color(c) => *c,
            _ => {
                warn_once!("Failed to convert PropertyValue {} to Color", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for OverflowAxis {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "visible" => OverflowAxis::Visible,
            PropertyValue::String(s) if s == "clip" => OverflowAxis::Clip,
            PropertyValue::String(s) if s == "hidden" => OverflowAxis::Hidden,
            PropertyValue::String(s) if s == "scroll" => OverflowAxis::Scroll,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to OverflowAxis",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for Display {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "flex" => Display::Flex,
            PropertyValue::String(s) if s == "grid" => Display::Grid,
            PropertyValue::String(s) if s == "block" => Display::Block,
            PropertyValue::String(s) if s == "none" => Display::None,
            _ => {
                warn!("Failed to convert PropertyValue {} to Display", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for BoxSizing {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "border-box" => BoxSizing::BorderBox,
            PropertyValue::String(s) if s == "content-box" => BoxSizing::ContentBox,
            _ => {
                warn!("Failed to convert PropertyValue {} to BoxSizing", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for PositionType {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "relative" => PositionType::Relative,
            PropertyValue::String(s) if s == "absolute" => PositionType::Absolute,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to PositionType",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for AlignItems {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "default" => AlignItems::Default,
            PropertyValue::String(s) if s == "start" => AlignItems::Start,
            PropertyValue::String(s) if s == "end" => AlignItems::End,
            PropertyValue::String(s) if s == "flex-start" => AlignItems::FlexStart,
            PropertyValue::String(s) if s == "flex-end" => AlignItems::FlexEnd,
            PropertyValue::String(s) if s == "center" => AlignItems::Center,
            PropertyValue::String(s) if s == "baseline" => AlignItems::Baseline,
            PropertyValue::String(s) if s == "stretch" => AlignItems::Stretch,
            _ => {
                warn!("Failed to convert PropertyValue {} to AlignItems", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for JustifyItems {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "default" => JustifyItems::Default,
            PropertyValue::String(s) if s == "start" => JustifyItems::Start,
            PropertyValue::String(s) if s == "end" => JustifyItems::End,
            PropertyValue::String(s) if s == "center" => JustifyItems::Center,
            PropertyValue::String(s) if s == "baseline" => JustifyItems::Baseline,
            PropertyValue::String(s) if s == "stretch" => JustifyItems::Stretch,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to JustifyItems",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for AlignSelf {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "auto" => AlignSelf::Auto,
            PropertyValue::String(s) if s == "start" => AlignSelf::Start,
            PropertyValue::String(s) if s == "end" => AlignSelf::End,
            PropertyValue::String(s) if s == "flex-start" => AlignSelf::FlexStart,
            PropertyValue::String(s) if s == "flex-end" => AlignSelf::FlexEnd,
            PropertyValue::String(s) if s == "center" => AlignSelf::Center,
            PropertyValue::String(s) if s == "baseline" => AlignSelf::Baseline,
            PropertyValue::String(s) if s == "stretch" => AlignSelf::Stretch,
            _ => {
                warn!("Failed to convert PropertyValue {} to AlignSelf", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for JustifySelf {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "auto" => JustifySelf::Auto,
            PropertyValue::String(s) if s == "start" => JustifySelf::Start,
            PropertyValue::String(s) if s == "end" => JustifySelf::End,
            PropertyValue::String(s) if s == "center" => JustifySelf::Center,
            PropertyValue::String(s) if s == "baseline" => JustifySelf::Baseline,
            PropertyValue::String(s) if s == "stretch" => JustifySelf::Stretch,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to JustifySelf",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for AlignContent {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "default" => AlignContent::Default,
            PropertyValue::String(s) if s == "start" => AlignContent::Start,
            PropertyValue::String(s) if s == "end" => AlignContent::End,
            PropertyValue::String(s) if s == "flex-start" => AlignContent::FlexStart,
            PropertyValue::String(s) if s == "flex-end" => AlignContent::FlexEnd,
            PropertyValue::String(s) if s == "center" => AlignContent::Center,
            PropertyValue::String(s) if s == "stretch" => AlignContent::Stretch,
            PropertyValue::String(s) if s == "space-between" => AlignContent::SpaceBetween,
            PropertyValue::String(s) if s == "space-around" => AlignContent::SpaceAround,
            PropertyValue::String(s) if s == "space-evenly" => AlignContent::SpaceEvenly,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to AlignContent",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for JustifyContent {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "default" => JustifyContent::Default,
            PropertyValue::String(s) if s == "start" => JustifyContent::Start,
            PropertyValue::String(s) if s == "end" => JustifyContent::End,
            PropertyValue::String(s) if s == "flex-start" => JustifyContent::FlexStart,
            PropertyValue::String(s) if s == "flex-end" => JustifyContent::FlexEnd,
            PropertyValue::String(s) if s == "center" => JustifyContent::Center,
            PropertyValue::String(s) if s == "stretch" => JustifyContent::Stretch,
            PropertyValue::String(s) if s == "space-between" => JustifyContent::SpaceBetween,
            PropertyValue::String(s) if s == "space-around" => JustifyContent::SpaceAround,
            PropertyValue::String(s) if s == "space-evenly" => JustifyContent::SpaceEvenly,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to JustifyContent",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for f32 {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::Number(n) => *n as f32,
            _ => {
                warn!("Failed to convert PropertyValue {} to f32", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for bool {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::Bool(b) => *b,
            _ => {
                warn!("Failed to convert PropertyValue {} to bool", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for OverflowClipBox {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "content-box" => OverflowClipBox::ContentBox,
            PropertyValue::String(s) if s == "padding-box" => OverflowClipBox::PaddingBox,
            PropertyValue::String(s) if s == "border-box" => OverflowClipBox::BorderBox,
            _ => {
                warn!("Failed to convert PropertyValue {} to u8", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for Option<f32> {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::Number(n) if *n >= 0.0 => Some(*n as f32),
            _ => None,
        }
    }
}

impl From<&PropertyValue> for FlexDirection {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "row" => FlexDirection::Row,
            PropertyValue::String(s) if s == "column" => FlexDirection::Column,
            PropertyValue::String(s) if s == "row-reverse" => FlexDirection::RowReverse,
            PropertyValue::String(s) if s == "column-reverse" => FlexDirection::ColumnReverse,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to FlexDirection",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for FlexWrap {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "nowrap" => FlexWrap::NoWrap,
            PropertyValue::String(s) if s == "wrap" => FlexWrap::Wrap,
            PropertyValue::String(s) if s == "wrap-reverse" => FlexWrap::WrapReverse,
            _ => {
                warn!("Failed to convert PropertyValue {} to FlexWrap", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for GridAutoFlow {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "row" => GridAutoFlow::Row,
            PropertyValue::String(s) if s == "column" => GridAutoFlow::Column,
            PropertyValue::String(s) if s == "row-dense" => GridAutoFlow::RowDense,
            PropertyValue::String(s) if s == "column-dense" => GridAutoFlow::ColumnDense,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to GridAutoFlow",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for String {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) => s.clone(),
            _ => {
                warn!("Failed to convert PropertyValue {} to String", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for LineHeight {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::Number(n) => LineHeight::Px(*n as f32),
            PropertyValue::Pixels(n) => LineHeight::Px(*n as f32),
            PropertyValue::Percent(n) => LineHeight::RelativeToFont(*n as f32 / 100.0),
            _ => {
                warn!("Failed to convert PropertyValue {} to LineHeight", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for FontSmoothing {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "none" => FontSmoothing::None,
            PropertyValue::String(s) if s == "antialiased" => FontSmoothing::AntiAliased,
            _ => {
                warn!(
                    "Failed to convert PropertyValue {} to FontSmoothing",
                    property
                );
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for Justify {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "left" => Justify::Left,
            PropertyValue::String(s) if s == "right" => Justify::Right,
            PropertyValue::String(s) if s == "center" => Justify::Center,
            PropertyValue::String(s) if s == "justified" => Justify::Justified,
            _ => {
                warn!("Failed to convert PropertyValue {} to Justify", property);
                Self::default()
            }
        }
    }
}

impl From<&PropertyValue> for LineBreak {
    fn from(property: &PropertyValue) -> Self {
        match property {
            PropertyValue::String(s) if s == "word" => LineBreak::WordBoundary,
            PropertyValue::String(s) if s == "char" => LineBreak::AnyCharacter,
            PropertyValue::String(s) if s == "word-or-char" => LineBreak::WordOrCharacter,
            PropertyValue::String(s) if s == "nowrap" => LineBreak::NoWrap,
            _ => {
                warn!("Failed to convert PropertyValue {} to LineBreak", property);
                Self::default()
            }
        }
    }
}
