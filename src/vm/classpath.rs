//! This module implements the the classpath context for NekoMaid.

use bevy::platform::collections::HashSet;

use crate::vm::allocator::{NekoClass, NekoWidget};
use crate::vm::style::{Selector, SelectorHierarchy};

/// Defines a widget's class path. A widget's class path can used to quickly
/// match selectors in stylesheets.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassPath {
    /// The hierarchy of widget classes from the root to the current widget.
    pub(super) hierarchy: Vec<WidgetClasses>,
}

impl ClassPath {
    /// Creates a new [`ClassPath`] instance.
    pub fn new(widget: WidgetClasses) -> Self {
        Self {
            hierarchy: vec![widget],
        }
    }

    /// Chains another class path onto the end of this one.
    pub fn chain(&mut self, other: &ClassPath) {
        self.hierarchy.extend_from_slice(&other.hierarchy);
    }

    /// Extends the class path hierarchy with a new widget.
    pub fn extend(&mut self, widget: WidgetClasses) {
        self.hierarchy.push(widget);
    }

    /// Returns the hierarchy of widget classes.
    ///
    /// The last element corresponds to the current widget.
    pub fn hierarchy(&self) -> &[WidgetClasses] {
        &self.hierarchy
    }

    /// Returns the depth of the class path hierarchy.
    pub fn depth(&self) -> usize {
        self.hierarchy.len()
    }

    /// Returns a reference to the [`WidgetClasses`] at the specified depth.
    ///
    /// The last element corresponds to the current widget, and depth 0 is the
    /// root.
    pub fn get_classes(&self, depth: usize) -> &WidgetClasses {
        &self.hierarchy[depth]
    }

    /// Returns a mutable reference to the [`WidgetClasses`] at the specified
    /// depth.
    ///
    /// The last element corresponds to the current widget, and depth 0 is the
    /// root.
    pub fn get_classes_mut(&mut self, depth: usize) -> &mut WidgetClasses {
        &mut self.hierarchy[depth]
    }

    /// Checks if the class path matches the given selector hierarchy.
    ///
    /// Selectors only match the deepest parts of the class path. (I.e, if the
    /// selector has three elements, only the last three elements of the class
    /// path are compared.)
    pub fn matches(&self, selector_hierarchy: &SelectorHierarchy) -> bool {
        if self.depth() < selector_hierarchy.depth() {
            return false;
        }

        let offset = self.depth() - selector_hierarchy.depth();
        for depth in 0 .. selector_hierarchy.depth() {
            let widget_classes = self.get_classes(depth + offset);
            let selector = selector_hierarchy.get_selector(depth);

            if !widget_classes.matches(selector) {
                return false;
            }
        }

        true
    }

    /// Checks if the class path partially matches the given selector hierarchy.
    /// Only widget types are compared; classes are ignored.
    pub fn partial_matches(&self, selector_hierarchy: &SelectorHierarchy) -> bool {
        if self.depth() < selector_hierarchy.depth() {
            return false;
        }

        let offset = self.depth() - selector_hierarchy.depth();
        for depth in 0 .. selector_hierarchy.depth() {
            let widget_classes = self.get_classes(depth + offset);
            let selector = selector_hierarchy.get_selector(depth);

            if selector.widget() != widget_classes.widget() {
                return false;
            }
        }

        true
    }

    /// Returns the [`WidgetClasses`] of the current widget.
    pub fn last(&self) -> &WidgetClasses {
        self.hierarchy.last().unwrap()
    }

    /// Returns a mutable reference to the [`WidgetClasses`] of the current
    /// widget.
    pub fn last_mut(&mut self) -> &mut WidgetClasses {
        self.hierarchy.last_mut().unwrap()
    }
}

/// Defines the classes associated with a single widget.
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetClasses {
    /// The widget type.
    pub(super) widget: NekoWidget,

    /// The classes associated with the widget.
    pub(super) classes: HashSet<NekoClass>,
}

impl WidgetClasses {
    /// Creates a new [`WidgetClasses`] instance for the given [`Widget`].
    pub fn new(widget: NekoWidget) -> Self {
        Self {
            widget,
            classes: HashSet::new(),
        }
    }

    /// Returns the [`Widget`] type.
    pub fn widget(&self) -> NekoWidget {
        self.widget
    }

    /// Returns the set of [`Class`]es associated with the [`Widget`].
    pub fn classes(&self) -> &HashSet<NekoClass> {
        &self.classes
    }

    /// Adds a [`Class`] to the set.
    pub fn add_class(&mut self, class: NekoClass) {
        self.classes.insert(class);
    }

    /// Removes a [`Class`] from the set.
    pub fn remove_class(&mut self, class: NekoClass) {
        self.classes.remove(&class);
    }

    /// Checks if the widget matches the given selector.
    pub fn matches(&self, selector: &Selector) -> bool {
        if self.widget != selector.widget() {
            return false;
        }

        for class in selector.with_classes() {
            if !self.classes.contains(class) {
                return false;
            }
        }

        for class in selector.without_classes() {
            if self.classes.contains(class) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::vm::allocator::NekoContextAllocator;

    #[test]
    fn test_widget_classes_matches() {
        let button = NekoContextAllocator::get_or_create_widget("button");
        let div = NekoContextAllocator::get_or_create_widget("div");
        let p = NekoContextAllocator::get_or_create_widget("p");
        let class_a = NekoContextAllocator::get_or_create_class("class-a");
        let class_b = NekoContextAllocator::get_or_create_class("class-b");

        let mut classpath = ClassPath::new(WidgetClasses::new(div));
        classpath.last_mut().add_class(class_a);

        classpath.extend(WidgetClasses::new(button));
        classpath.last_mut().add_class(class_b);

        classpath.extend(WidgetClasses::new(p));

        let mut selector_hierarchy = SelectorHierarchy::default();
        selector_hierarchy.extend(Selector::build(button, &[class_b], &[]));
        selector_hierarchy.extend(Selector::build(p, &[], &[]));

        assert!(classpath.matches(&selector_hierarchy));
    }

    #[test]
    fn test_partial_matches() {
        let button = NekoContextAllocator::get_or_create_widget("button");
        let div = NekoContextAllocator::get_or_create_widget("div");
        let p = NekoContextAllocator::get_or_create_widget("p");
        let class_a = NekoContextAllocator::get_or_create_class("class-a");
        let class_b = NekoContextAllocator::get_or_create_class("class-b");

        let mut classpath = ClassPath::new(WidgetClasses::new(div));
        classpath.extend(WidgetClasses::new(button));
        classpath.extend(WidgetClasses::new(p));

        let mut selector_hierarchy = SelectorHierarchy::default();
        selector_hierarchy.extend(Selector::build(p, &[class_a], &[class_b]));

        assert!(classpath.partial_matches(&selector_hierarchy));
    }
}
