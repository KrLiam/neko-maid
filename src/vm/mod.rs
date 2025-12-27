//! This module implements the NekoMaid context container.

use std::collections::HashMap;

use crate::parse::nodes::{LayoutNode, ModuleNode};
use crate::parse::token::TokenPosition;
use crate::vm::allocator::{NekoContextAllocator, NekoWidget};
use crate::vm::classpath::{ClassPath, WidgetClasses};
use crate::vm::context::NekoContext;
use crate::vm::element::NekoElement;
use crate::vm::properties::{PropertyValue, WidgetDefinition};
use crate::vm::style::{NekoStyle, Selector, SelectorHierarchy};

pub mod allocator;
pub mod classpath;
pub mod context;
pub mod element;
pub mod properties;
pub mod style;

#[cfg(test)]
mod test;

/// The NekoMaid virtual machine (VM) that manages contexts and widget
/// definitions.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NekoMaidVM {
    /// A mapping of widget identifiers to their definitions.
    pub(super) widgets: HashMap<NekoWidget, WidgetDefinition>,

    /// A list of contexts managed by the VM.
    pub(super) contexts: HashMap<String, NekoContext>,
}

impl NekoMaidVM {
    /// Registers a widget definition in the context.
    pub fn register_widget(&mut self, definition: WidgetDefinition) {
        self.widgets.insert(definition.widget(), definition);
    }

    /// Retrieves a widget definition by its identifier.
    pub fn get_widget_definition(&self, widget: NekoWidget) -> Option<&WidgetDefinition> {
        self.widgets.get(&widget)
    }

    /// Resolves a module by its name and associates it with a ModuleNode. The
    /// module is then stored in this VM to be imported by future modules. If
    /// there is already a module with the same name, it will be overwritten.
    ///
    /// The resulting ResolvedElementTree will be returned.
    ///
    /// If there are any errors during the resolution process, all errors will
    /// be collected and returned as a vector. In this case, the module will not
    /// be stored in the VM.
    pub fn resolve_module<S: Into<String>>(
        &mut self,
        module_name: S,
        module: ModuleNode,
    ) -> Result<Vec<NekoElement>, Vec<NekoMaidVMError>> {
        let mut context = NekoContext::default();
        let mut errors = Vec::new();

        // resolve imports
        for import in module.imports {
            let Some(imported_context) = self.contexts.get(&import.path) else {
                errors.push(NekoMaidVMError::ModuleNotFound(
                    import.path,
                    import.position,
                ));
                continue;
            };
            context.append(imported_context.clone());
        }

        // resolve variables
        for var in module.variables {
            let var_name = NekoContextAllocator::get_or_create_variable(var.name);
            let var_value = match PropertyValue::from_property_node_value(var.value, &context) {
                Ok(value) => value,
                Err(err) => {
                    errors.push(err);
                    continue;
                }
            };
            context.set_variable(var_name, var_value);
        }

        // resolve styles
        for style in module.styles {
            for resolved in NekoStyle::from_style_node(style, &context, self, &mut errors) {
                context.add_style(resolved);
            }
        }

        // resolve layout elements
        let mut elements = Vec::new();
        for layout in module.layouts {
            let el = resolve_layout_node_recursive(layout, None, &context, self, &mut errors);
            if let Some(el) = el {
                elements.push(el);
            }
        }

        // done
        if errors.is_empty() {
            self.contexts.insert(module_name.into(), context);
            Ok(elements)
        } else {
            Err(errors)
        }
    }
}

/// Errors that can occur when resolving modules in the NekoMaid VM.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum NekoMaidVMError {
    /// An error indicating that a module could not be found.
    #[error("Module not found: {0}, at {1}")]
    ModuleNotFound(String, TokenPosition),

    /// An error indicating that a variable could not be found.
    #[error("Variable not found: {0}, at {1}")]
    VariableNotFound(String, TokenPosition),

    /// An error indicating that an unknown widget was referenced.
    #[error("Unknown widget: {name}, at {position}")]
    UnknownWidget {
        /// The name of the unknown widget.
        name: String,

        /// The position where the error occurred.
        position: TokenPosition,
    },

    /// An error indicating that an invalid property was provided.
    #[error("Invalid property '{property}' for '{widget}' at {position}")]
    InvalidProperty {
        /// The widget where the error occurred.
        property: String,

        /// The widget where the error occurred.
        widget: String,

        /// The position where the error occurred.
        position: TokenPosition,
    },
}

fn resolve_layout_node_recursive(
    node: LayoutNode,
    classpath: Option<ClassPath>,
    ctx: &NekoContext,
    vm: &NekoMaidVM,
    errors: &mut Vec<NekoMaidVMError>,
) -> Option<NekoElement> {
    // resolve classpath
    let widget = NekoContextAllocator::get_or_create_widget(&node.widget);
    let Some(widget_def) = vm.get_widget_definition(widget) else {
        errors.push(NekoMaidVMError::UnknownWidget {
            name: node.widget,
            position: node.position,
        });
        return None;
    };

    let mut widget_classes = WidgetClasses::new(widget);
    for class in node.classes {
        let class_id = NekoContextAllocator::get_or_create_class(&class);
        widget_classes.add_class(class_id);
    }

    let classpath = match classpath {
        Some(mut path) => {
            path.extend(widget_classes);
            path
        }
        None => ClassPath::new(widget_classes),
    };

    let mut element = NekoElement::new(classpath);

    // import styles
    element.add_style(widget_def.default_style());

    for style in ctx.styles() {
        if element.classpath().partial_matches(style.selector()) {
            element.add_style(style.clone());
        }
    }

    // resolve properties
    if !node.properties.is_empty() {
        let mut selector_hierarchy = SelectorHierarchy::default();
        for hierarchy in element.classpath().hierarchy() {
            selector_hierarchy.extend(Selector::new(hierarchy.widget()));
        }

        let mut style = NekoStyle::new(selector_hierarchy);
        for property in node.properties {
            let property_name = NekoContextAllocator::get_or_create_property(&property.name);
            if widget_def.get_property(property_name).is_none() {
                errors.push(NekoMaidVMError::InvalidProperty {
                    property: property.name,
                    widget: node.widget.clone(),
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
        element.add_style(style);
    }

    // resolve children
    for child in node.children {
        let classpath = element.classpath().clone();
        let el = resolve_layout_node_recursive(child, Some(classpath), ctx, vm, errors);
        if let Some(el) = el {
            element.add_child(el);
        }
    }

    Some(element)
}
