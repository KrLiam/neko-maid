use bevy::color::Color;
use bevy::platform::collections::HashSet;
use common_macros::hash_map;
use pretty_assertions::assert_eq;

use crate::parse::parse_neko_ui;
use crate::vm::NekoMaidVM;
use crate::vm::allocator::NekoContextAllocator;
use crate::vm::classpath::{ClassPath, WidgetClasses};
use crate::vm::context::NekoContext;
use crate::vm::element::NekoElement;
use crate::vm::properties::{PropertyDefinition, PropertyValue, WidgetDefinition};
use crate::vm::style::{NekoStyle, Selector, SelectorHierarchy};

#[test]
fn resolve_nekomaid_ui() {
    const UI_SOURCE_1: &str = r#"
        var press_col: #ff0000;
        var hover_col: #00ff00;
        var down_col: #0000ff;

        style button {
            width: 100px;
            height: 50px;
            background-color: $press_col;
        }

        style button +hover {
            background-color: $hover_col;
        }

        style button +pressed {
            background-color: $down_col;
        }
    "#;

    const UI_SOURCE_2: &str = r#"
        import "UI_SOURCE_1";

        var down_col: #ffffff;

        layout div {
            +outer-menu;

            with button {
                border-color: $press_col;
                border-width: 2px;
            }
        }
    "#;

    let button = NekoContextAllocator::get_or_create_widget("button");
    let div = NekoContextAllocator::get_or_create_widget("div");

    let width_prop = NekoContextAllocator::get_or_create_property("width");
    let height_prop = NekoContextAllocator::get_or_create_property("height");
    let bg_color_prop = NekoContextAllocator::get_or_create_property("background-color");
    let border_color_prop = NekoContextAllocator::get_or_create_property("border-color");
    let border_width_prop = NekoContextAllocator::get_or_create_property("border-width");
    let border_radius_prop = NekoContextAllocator::get_or_create_property("border-radius");

    let outer_menu_class = NekoContextAllocator::get_or_create_class("outer-menu");
    let pressed_class = NekoContextAllocator::get_or_create_class("pressed");
    let hover_class = NekoContextAllocator::get_or_create_class("hover");

    let press_col_var = NekoContextAllocator::get_or_create_variable("press_col");
    let hover_col_var = NekoContextAllocator::get_or_create_variable("hover_col");
    let down_col_var = NekoContextAllocator::get_or_create_variable("down_col");

    let red = PropertyValue::Color(Color::srgb(1.0, 0.0, 0.0));
    let green = PropertyValue::Color(Color::srgb(0.0, 1.0, 0.0));
    let blue = PropertyValue::Color(Color::srgb(0.0, 0.0, 1.0));
    let white = PropertyValue::Color(Color::srgb(1.0, 1.0, 1.0));
    let transparent = PropertyValue::Color(Color::NONE);

    fn px(num: f64) -> PropertyValue {
        PropertyValue::Pixels(num)
    }

    let mut vm = NekoMaidVM::default();
    vm.register_widget(WidgetDefinition {
        widget: div,
        properties: hash_map! {
            width_prop => PropertyDefinition::new(width_prop, "auto"),
            height_prop => PropertyDefinition::new(height_prop, "auto"),
            bg_color_prop => PropertyDefinition::new(bg_color_prop, transparent.clone()),
            border_color_prop => PropertyDefinition::new(border_color_prop, transparent.clone()),
            border_width_prop => PropertyDefinition::new(border_width_prop, px(0.0)),
            border_radius_prop => PropertyDefinition::new(border_radius_prop, px(0.0)),
        },
    });
    vm.register_widget(WidgetDefinition {
        widget: button,
        properties: hash_map! {
            width_prop => PropertyDefinition::new(width_prop, "auto"),
            height_prop => PropertyDefinition::new(height_prop, "auto"),
            bg_color_prop => PropertyDefinition::new(bg_color_prop, transparent.clone()),
            border_color_prop => PropertyDefinition::new(border_color_prop, transparent.clone()),
            border_width_prop => PropertyDefinition::new(border_width_prop, px(0.0)),
            border_radius_prop => PropertyDefinition::new(border_radius_prop, px(0.0)),
        },
    });

    let resolved = NekoElement {
        widget: div,
        classpath: ClassPath {
            hierarchy: vec![WidgetClasses {
                widget: div,
                classes: HashSet::from([outer_menu_class]),
            }],
        },
        styles: vec![
            // default style
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: div,
                        with_classes: HashSet::new(),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    width_prop => "auto".into(),
                    height_prop => "auto".into(),
                    bg_color_prop => transparent.clone(),
                    border_color_prop => transparent.clone(),
                    border_width_prop => px(0.0),
                    border_radius_prop => px(0.0),
                },
            },
        ],
        children: vec![NekoElement {
            widget: button,
            classpath: ClassPath {
                hierarchy: vec![
                    WidgetClasses {
                        widget: div,
                        classes: HashSet::from([outer_menu_class]),
                    },
                    WidgetClasses {
                        widget: button,
                        classes: HashSet::new(),
                    },
                ],
            },
            styles: vec![
                // layout style
                NekoStyle {
                    selector: SelectorHierarchy {
                        selectors: vec![
                            Selector {
                                widget: div,
                                with_classes: HashSet::new(),
                                without_classes: HashSet::new(),
                            },
                            Selector {
                                widget: button,
                                with_classes: HashSet::new(),
                                without_classes: HashSet::new(),
                            },
                        ],
                    },
                    properties: hash_map! {
                        border_color_prop => red.clone(),
                        border_width_prop => px(2.0),
                    },
                },
                // pressed style
                NekoStyle {
                    selector: SelectorHierarchy {
                        selectors: vec![Selector {
                            widget: button,
                            with_classes: HashSet::from([pressed_class]),
                            without_classes: HashSet::new(),
                        }],
                    },
                    properties: hash_map! {
                        bg_color_prop => blue.clone(),
                    },
                },
                // hover style
                NekoStyle {
                    selector: SelectorHierarchy {
                        selectors: vec![Selector {
                            widget: button,
                            with_classes: HashSet::from([hover_class]),
                            without_classes: HashSet::new(),
                        }],
                    },
                    properties: hash_map! {
                        bg_color_prop => green.clone(),
                    },
                },
                // button style
                NekoStyle {
                    selector: SelectorHierarchy {
                        selectors: vec![Selector {
                            widget: button,
                            with_classes: HashSet::new(),
                            without_classes: HashSet::new(),
                        }],
                    },
                    properties: hash_map! {
                        width_prop => px(100.0),
                        height_prop => px(50.0),
                        bg_color_prop => red.clone(),
                    },
                },
                // default style
                NekoStyle {
                    selector: SelectorHierarchy {
                        selectors: vec![Selector {
                            widget: button,
                            with_classes: HashSet::new(),
                            without_classes: HashSet::new(),
                        }],
                    },
                    properties: hash_map! {
                        width_prop => "auto".into(),
                        height_prop => "auto".into(),
                        bg_color_prop => transparent.clone(),
                        border_color_prop => transparent.clone(),
                        border_width_prop => px(0.0),
                        border_radius_prop => px(0.0),
                    },
                },
            ],
            children: vec![],
        }],
    };

    let src1_ctx = NekoContext {
        variables: hash_map! {
            press_col_var => red.clone(),
            hover_col_var => green.clone(),
            down_col_var => blue.clone(),
        },
        styles: vec![
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: button,
                        with_classes: HashSet::new(),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    width_prop => px(100.0),
                    height_prop => px(50.0),
                    bg_color_prop => red.clone(),
                },
            },
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: button,
                        with_classes: HashSet::from([hover_class]),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    bg_color_prop => green.clone(),
                },
            },
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: button,
                        with_classes: HashSet::from([pressed_class]),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    bg_color_prop => blue.clone(),
                },
            },
        ],
    };

    let src2_ctx = NekoContext {
        variables: hash_map! {
            press_col_var => red.clone(),
            hover_col_var => green.clone(),
            down_col_var => white.clone(),
        },
        styles: vec![
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: button,
                        with_classes: HashSet::new(),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    width_prop => px(100.0),
                    height_prop => px(50.0),
                    bg_color_prop => red.clone(),
                },
            },
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: button,
                        with_classes: HashSet::from([hover_class]),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    bg_color_prop => green.clone(),
                },
            },
            NekoStyle {
                selector: SelectorHierarchy {
                    selectors: vec![Selector {
                        widget: button,
                        with_classes: HashSet::from([pressed_class]),
                        without_classes: HashSet::new(),
                    }],
                },
                properties: hash_map! {
                    bg_color_prop => blue.clone(),
                },
            },
        ],
    };

    let src1 = parse_neko_ui(UI_SOURCE_1).unwrap();
    vm.resolve_module("UI_SOURCE_1", src1).unwrap();
    assert_eq!(vm.contexts.get("UI_SOURCE_1").unwrap(), &src1_ctx);

    let src2 = parse_neko_ui(UI_SOURCE_2).unwrap();
    let layout = vm.resolve_module("UI_SOURCE_2", src2).unwrap();
    assert_eq!(vm.contexts.get("UI_SOURCE_2").unwrap(), &src2_ctx);

    assert_eq!(layout, vec![resolved]);
}
