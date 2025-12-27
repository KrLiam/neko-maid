//! Defines the context allocator for NekoMaid UI widgets, properties, and
//! classes.

use std::sync::Mutex;

use bevy::platform::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    /// A global mutex for the context allocator.
    static ref ALLOCATOR_MUTEX: Mutex<NekoContextAllocator> = Mutex::new(NekoContextAllocator::new());
}

/// A NekoMaid widget identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NekoWidget(u64);

/// A NekoMaid property identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NekoProperty(u64);

/// A NekoMaid class identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NekoClass(u64);

/// A NekoMaid variable identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NekoVariable(u64);

/// The context allocator for NekoMaid UI.
#[derive(Debug)]
pub struct NekoContextAllocator {
    /// An incrementor for generating unique identifiers.
    incrementor: u64,

    /// A mapping of widget names to their identifiers.
    widgets: HashMap<String, NekoWidget>,

    /// A mapping of property names to their identifiers.
    properties: HashMap<String, NekoProperty>,

    /// A mapping of class names to their identifiers.
    classes: HashMap<String, NekoClass>,

    /// A mapping of variable names to their identifiers.
    variable: HashMap<String, NekoVariable>,
}

impl NekoContextAllocator {
    /// Creates a new, empty ContextAllocator.
    fn new() -> Self {
        Self {
            incrementor: 0,
            widgets: HashMap::new(),
            properties: HashMap::new(),
            classes: HashMap::new(),
            variable: HashMap::new(),
        }
    }

    /// Retrieves a [`Widget`] identifier by its name, if it exists.
    pub fn get_widget<S: AsRef<str>>(name: S) -> Option<NekoWidget> {
        let allocator = &ALLOCATOR_MUTEX.lock().unwrap();
        allocator.widgets.get(name.as_ref()).copied()
    }

    /// Retrieves a [`Property`] identifier by its name, if it exists.
    pub fn get_property<S: AsRef<str>>(name: S) -> Option<NekoProperty> {
        let allocator = &ALLOCATOR_MUTEX.lock().unwrap();
        allocator.properties.get(name.as_ref()).copied()
    }

    /// Retrieves a [`Class`] identifier by its name, if it exists.
    pub fn get_class<S: AsRef<str>>(name: S) -> Option<NekoClass> {
        let allocator = &ALLOCATOR_MUTEX.lock().unwrap();
        allocator.classes.get(name.as_ref()).copied()
    }

    /// Retrieves a [`Variable`] identifier by its name, if it exists.
    pub fn get_variable<S: AsRef<str>>(name: S) -> Option<NekoVariable> {
        let allocator = &ALLOCATOR_MUTEX.lock().unwrap();
        allocator.variable.get(name.as_ref()).copied()
    }

    /// Retrieves an existing [`Widget`] identifier by its name, or creates a
    /// new one if it does not exist.
    pub fn get_or_create_widget<S: AsRef<str>>(name: S) -> NekoWidget {
        let allocator = &mut ALLOCATOR_MUTEX.lock().unwrap();

        let name_ref = name.as_ref();
        if let Some(widget) = allocator.widgets.get(name_ref) {
            return *widget;
        }

        allocator.incrementor += 1;
        let widget = NekoWidget(allocator.incrementor);
        allocator.widgets.insert(name_ref.to_string(), widget);
        widget
    }

    /// Retrieves an existing [`Property`] identifier by its name, or creates a
    /// new one if it does not exist.
    pub fn get_or_create_property<S: AsRef<str>>(name: S) -> NekoProperty {
        let allocator = &mut ALLOCATOR_MUTEX.lock().unwrap();

        let name_ref = name.as_ref();
        if let Some(property) = allocator.properties.get(name_ref) {
            return *property;
        }

        allocator.incrementor += 1;
        let property = NekoProperty(allocator.incrementor);
        allocator.properties.insert(name_ref.to_string(), property);
        property
    }

    /// Retrieves an existing [`Class`] identifier by its name, or creates a new
    /// one if it does not exist.
    pub fn get_or_create_class<S: AsRef<str>>(name: S) -> NekoClass {
        let allocator = &mut ALLOCATOR_MUTEX.lock().unwrap();

        let name_ref = name.as_ref();
        if let Some(class) = allocator.classes.get(name_ref) {
            return *class;
        }

        allocator.incrementor += 1;
        let class = NekoClass(allocator.incrementor);
        allocator.classes.insert(name_ref.to_string(), class);
        class
    }

    /// Retrieves an existing [`Variable`] identifier by its name, or creates a
    /// new one if it does not exist.
    pub fn get_or_create_variable<S: AsRef<str>>(name: S) -> NekoVariable {
        let allocator = &mut ALLOCATOR_MUTEX.lock().unwrap();

        let name_ref = name.as_ref();
        if let Some(variable) = allocator.variable.get(name_ref) {
            return *variable;
        }

        allocator.incrementor += 1;
        let variable = NekoVariable(allocator.incrementor);
        allocator.variable.insert(name_ref.to_string(), variable);
        variable
    }
}
