//! A NekoMaid context container and related structures.

use std::collections::HashMap;

use crate::vm::allocator::NekoVariable;
use crate::vm::properties::PropertyValue;
use crate::vm::style::NekoStyle;

/// A NekoMaid context container.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NekoContext {
    /// A mapping of variable identifiers to their values.
    pub(super) variables: HashMap<NekoVariable, PropertyValue>,

    /// A list of style definitions.
    pub(super) styles: Vec<NekoStyle>,
}

impl NekoContext {
    /// Sets a variable in the context, overwriting any existing value.
    pub fn set_variable(&mut self, variable: NekoVariable, value: PropertyValue) -> NekoVariable {
        self.variables.insert(variable, value);
        variable
    }

    /// Retrieves a variable's current value from the context.
    ///
    /// Returns `None` if the variable is not found in this context.
    pub fn get_variable(&self, variable: NekoVariable) -> Option<&PropertyValue> {
        self.variables.get(&variable)
    }

    /// Adds a style definition to the context.
    ///
    /// If there is already an existing style with the same selector hierarchy,
    /// the two styles will be merged, overwriting any conflicting properties
    /// with those from the new style.
    ///
    /// Styles added later have higher precedence when applying styles at
    /// runtime.
    pub fn add_style(&mut self, style: NekoStyle) {
        for existing_style in &mut self.styles {
            if existing_style.selector() != style.selector() {
                continue;
            }

            for (property, value) in style.into_properties() {
                existing_style.set_property(property, value);
            }
            return;
        }

        self.styles.push(style);
    }

    /// Appends another context into this one, merging their contents.
    pub fn append(&mut self, other: NekoContext) {
        for (variable, value) in other.variables.into_iter() {
            self.set_variable(variable, value);
        }

        for style in other.styles.into_iter() {
            self.add_style(style);
        }
    }

    /// Gets a reference to the styles defined in this context.
    ///
    /// Styles added later have higher precedence when applying styles at
    /// runtime.
    pub fn styles(&self) -> &[NekoStyle] {
        &self.styles
    }
}
