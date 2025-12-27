//! A finalized data structure for representing individual UI elements.

use crate::vm::allocator::NekoWidget;
use crate::vm::classpath::ClassPath;
use crate::vm::style::NekoStyle;

/// A resolve UI element, ready to be created in Bevy.
#[derive(Debug, Clone, PartialEq)]
pub struct NekoElement {
    /// The widget identifier for this element.
    pub(super) widget: NekoWidget,

    /// The class path associated with this element.
    pub(super) classpath: ClassPath,

    /// A list of styles associated with this element. Higher values have
    /// greater precedence.
    pub(super) styles: Vec<NekoStyle>,

    /// A list of child elements.
    pub(super) children: Vec<NekoElement>,
}

impl NekoElement {
    /// Creates a new NekoElement with the given widget identifier.
    pub fn new(classpath: ClassPath) -> Self {
        Self {
            widget: classpath.last().widget(),
            classpath,
            styles: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Returns the widget identifier for this element.
    pub fn widget(&self) -> NekoWidget {
        self.widget
    }

    /// Adds a style to the element. This style will have the highest
    /// precedence.
    pub fn add_style(&mut self, style: NekoStyle) {
        self.styles.insert(0, style);
    }

    /// Adds a child element to this element.
    ///
    /// Note: This method *does not* update the child's classpath.
    pub fn add_child(&mut self, element: NekoElement) {
        self.children.push(element);
    }

    /// Returns the class path associated with this element.
    pub fn classpath(&self) -> &ClassPath {
        &self.classpath
    }

    /// Returns the styles associated with this element.
    ///
    /// Styles are ordered from highest to lowest precedence.
    pub fn styles(&self) -> &[NekoStyle] {
        &self.styles
    }

    /// Returns the child elements of this element.
    pub fn children(&self) -> &[NekoElement] {
        &self.children
    }
}
