//! Defines styles and selectors for the NekoMaid virtual machine.

use std::collections::HashMap;

use bevy::platform::collections::HashSet;

use crate::parse::nodes::{SelectorPart, StyleNode};
use crate::vm::allocator::{NekoClass, NekoContextAllocator, NekoProperty, NekoWidget};
use crate::vm::context::NekoContext;
use crate::vm::properties::PropertyValue;
use crate::vm::{NekoMaidVM, NekoMaidVMError};

/// A style definition in the NekoMaid context.
#[derive(Debug, Clone, PartialEq)]
pub struct NekoStyle {
    /// The selector hierarchy for this style.
    pub(super) selector: SelectorHierarchy,

    /// The properties defined in this style.
    pub(super) properties: HashMap<NekoProperty, PropertyValue>,
}

impl NekoStyle {
    /// Creates a new NekoStyle instance.
    pub fn new(selector: SelectorHierarchy) -> Self {
        Self {
            selector,
            properties: HashMap::new(),
        }
    }

    /// Returns a reference to the selector hierarchy of this style.
    pub fn selector(&self) -> &SelectorHierarchy {
        &self.selector
    }

    /// Returns a reference to the properties defined in this style.
    pub fn properties(&self) -> &HashMap<NekoProperty, PropertyValue> {
        &self.properties
    }

    /// Consumes this style and returns its properties.
    pub fn into_properties(self) -> HashMap<NekoProperty, PropertyValue> {
        self.properties
    }

    /// Sets a property in this style.
    pub fn set_property<V: Into<PropertyValue>>(&mut self, property: NekoProperty, value: V) {
        self.properties.insert(property, value.into());
    }

    /// Retrieves a property's value from this style.
    pub fn get_property(&self, property: NekoProperty) -> Option<&PropertyValue> {
        self.properties.get(&property)
    }

    /// Converts a [`StyleNode`] into a list of [`NekoStyle`]s.
    ///
    /// If there are any errors during the conversion process, they will be
    /// collected in the provided errors vector.
    pub fn from_style_node(
        style_node: StyleNode,
        ctx: &NekoContext,
        vm: &NekoMaidVM,
        errors: &mut Vec<NekoMaidVMError>,
    ) -> Vec<Self> {
        let mut styles = Vec::new();
        let selector_hierarchy = SelectorHierarchy::default();
        build_styles_recursive(style_node, selector_hierarchy, &mut styles, ctx, vm, errors);
        styles
    }
}

/// Defines a hierarchy of selectors for matching against a ClassPath.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SelectorHierarchy {
    /// The list of selectors in the hierarchy.
    pub(super) selectors: Vec<Selector>,
}

impl SelectorHierarchy {
    /// Creates a new SelectorHierarchy instance.
    pub fn new(selectors: Vec<Selector>) -> Self {
        Self { selectors }
    }

    /// Creates a SelectorHierarchy from a single widget.
    pub fn from(widget: NekoWidget) -> Self {
        Self {
            selectors: vec![Selector::new(widget)],
        }
    }

    /// Extends the selector hierarchy with a new selector.
    pub fn extend(&mut self, selector: Selector) {
        self.selectors.push(selector);
    }

    /// Returns the list of selectors in the hierarchy.
    pub fn selectors(&self) -> &[Selector] {
        &self.selectors
    }

    /// Returns the depth of the selector hierarchy.
    pub fn depth(&self) -> usize {
        self.selectors.len()
    }

    /// Returns a reference to the [`Selector`] at the specified depth.
    pub fn get_selector(&self, depth: usize) -> &Selector {
        &self.selectors[depth]
    }
}

/// Defines a selector used for matching against a ClassPath.
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    /// The [`Widget`] type to match.
    pub(super) widget: NekoWidget,

    /// [`Class`]es that must be present for a match.
    pub(super) with_classes: HashSet<NekoClass>,

    /// [`Class`]es that must be absent for a match.
    pub(super) without_classes: HashSet<NekoClass>,
}

impl Selector {
    /// Creates a new Selector instance.
    pub fn new(widget: NekoWidget) -> Self {
        Self {
            widget,
            with_classes: HashSet::new(),
            without_classes: HashSet::new(),
        }
    }

    /// Creates a new Selector instance with the specified widget and class
    /// sets.
    pub fn build(
        widget: NekoWidget,
        with_classes: &[NekoClass],
        without_classes: &[NekoClass],
    ) -> Self {
        Self {
            widget,
            with_classes: with_classes.iter().cloned().collect(),
            without_classes: without_classes.iter().cloned().collect(),
        }
    }

    /// Returns the [`Widget`] type to match.
    pub fn widget(&self) -> NekoWidget {
        self.widget
    }

    /// Returns the set of classes that must be present for a match.
    pub fn with_classes(&self) -> &HashSet<NekoClass> {
        &self.with_classes
    }

    /// Returns the set of classes that must be absent for a match.
    pub fn without_classes(&self) -> &HashSet<NekoClass> {
        &self.without_classes
    }

    /// Adds a class that must be present for a match.
    pub fn add_with_class(&mut self, class: NekoClass) {
        self.with_classes.insert(class);
    }

    /// Adds a class that must be absent for a match.
    pub fn add_without_class(&mut self, class: NekoClass) {
        self.without_classes.insert(class);
    }
}

/// Recursively builds styles from a StyleNode and its children.
fn build_styles_recursive(
    node: StyleNode,
    mut selector_hierarchy: SelectorHierarchy,
    styles: &mut Vec<NekoStyle>,
    ctx: &NekoContext,
    vm: &NekoMaidVM,
    errors: &mut Vec<NekoMaidVMError>,
) {
    // build selector
    let widget = NekoContextAllocator::get_or_create_widget(&node.selector.widget);
    let mut selector = Selector::new(widget);

    let Some(widget_def) = vm.get_widget_definition(widget) else {
        errors.push(NekoMaidVMError::UnknownWidget {
            name: node.selector.widget,
            position: node.selector.position,
        });
        return;
    };

    for part in node.selector.parts {
        match part {
            SelectorPart::WithClass(c) => {
                let c = NekoContextAllocator::get_or_create_class(c);
                selector.add_with_class(c);
            }
            SelectorPart::WithoutClass(c) => {
                let c = NekoContextAllocator::get_or_create_class(c);
                selector.add_without_class(c);
            }
        }
    }

    selector_hierarchy.selectors.push(selector);

    // process children
    for child in node.children {
        build_styles_recursive(child, selector_hierarchy.clone(), styles, ctx, vm, errors);
    }

    // gather properties
    if !node.properties.is_empty() {
        let mut style = NekoStyle::new(selector_hierarchy);

        for property in node.properties {
            let property_name = NekoContextAllocator::get_or_create_property(&property.name);
            if widget_def.get_property(property_name).is_none() {
                errors.push(NekoMaidVMError::InvalidProperty {
                    property: property.name,
                    widget: node.selector.widget.clone(),
                    position: property.position,
                });
                continue;
            }

            let property_value = match PropertyValue::from_property_node_value(property.value, ctx)
            {
                Ok(v) => v,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };
            style.set_property(property_name, property_value);
        }

        styles.push(style);
    }
}

#[cfg(test)]
mod tests {
    use bevy::color::Color;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::parse::nodes::{PropertyNode, PropertyNodeValue, SelectorNode};
    use crate::vm::properties::{PropertyDefinition, WidgetDefinition};

    #[test]
    fn from_style_node() {
        let style_node = StyleNode {
            selector: SelectorNode {
                widget: "div".to_string(),
                parts: vec![SelectorPart::WithClass("container".to_string())],
                position: Default::default(),
            },
            properties: vec![
                PropertyNode {
                    name: "bg-color".to_string(),
                    value: PropertyNodeValue::Color(Color::srgb(1.0, 1.0, 1.0)),
                    position: Default::default(),
                },
                PropertyNode {
                    name: "border-color".to_string(),
                    value: PropertyNodeValue::Color(Color::srgb(1.0, 0.0, 0.0)),
                    position: Default::default(),
                },
            ],
            children: vec![StyleNode {
                selector: SelectorNode {
                    widget: "button".to_string(),
                    parts: vec![
                        SelectorPart::WithClass("hover".to_string()),
                        SelectorPart::WithoutClass("pressed".to_string()),
                    ],
                    position: Default::default(),
                },
                properties: vec![PropertyNode {
                    name: "bg-color".to_string(),
                    value: PropertyNodeValue::Color(Color::srgb(0.0, 1.0, 0.0)),
                    position: Default::default(),
                }],
                children: vec![],
            }],
        };

        let div = NekoContextAllocator::get_or_create_widget("div");
        let button = NekoContextAllocator::get_or_create_widget("button");
        let container_class = NekoContextAllocator::get_or_create_class("container");
        let hover_class = NekoContextAllocator::get_or_create_class("hover");
        let pressed_class = NekoContextAllocator::get_or_create_class("pressed");
        let bg_color_prop = NekoContextAllocator::get_or_create_property("bg-color");
        let border_color_prop = NekoContextAllocator::get_or_create_property("border-color");

        let black = PropertyValue::Color(Color::srgb(0.0, 0.0, 0.0));

        let mut vm = NekoMaidVM::default();
        vm.register_widget(WidgetDefinition {
            widget: div,
            properties: hash_map! {
                bg_color_prop => PropertyDefinition::new(bg_color_prop, black.clone()),
                border_color_prop => PropertyDefinition::new(border_color_prop, black.clone()),
            },
        });
        vm.register_widget(WidgetDefinition {
            widget: button,
            properties: hash_map! {
                bg_color_prop => PropertyDefinition::new(bg_color_prop, black.clone()),
                border_color_prop => PropertyDefinition::new(border_color_prop, black.clone()),
            },
        });

        let mut errors = Vec::new();
        let styles =
            NekoStyle::from_style_node(style_node, &NekoContext::default(), &vm, &mut errors);
        assert_eq!(errors, vec![]);

        let resolved = vec![
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![
                        Selector {
                            widget: div,
                            with_classes: HashSet::from([container_class]),
                            without_classes: HashSet::new(),
                        },
                        Selector {
                            widget: button,
                            with_classes: HashSet::from([hover_class]),
                            without_classes: HashSet::from([pressed_class]),
                        },
                    ],
                },
                properties: hash_map! {
                    bg_color_prop => PropertyValue::Color(Color::srgb(0.0, 1.0, 0.0)),
                },
            },
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: div,
                        with_classes: HashSet::from([container_class]),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    bg_color_prop => PropertyValue::Color(Color::srgb(1.0, 1.0, 1.0)),
                    border_color_prop => PropertyValue::Color(Color::srgb(1.0, 0.0, 0.0)),
                },
            },
        ];
        assert_eq!(styles, resolved);
    }
}
