//! Defines the native widgets available in NekoMaid UI.

use std::sync::Arc;

use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::parse::element::NekoElement;
use crate::parse::value::PropertyValue;
use crate::parse::widget::NativeWidget;

lazy_static! {
    /// The list of native widgets available in NekoMaid UI.
    pub static ref NATIVE_WIDGETS: Vec<NativeWidget> = vec![
        NativeWidget {
            name: String::from("div"),
            default_properties: {
                let mut m = HashMap::new();
                node_properties(&mut m);
                background_color_properties(&mut m);
                border_color_properties(&mut m);
                border_radius_properties(&mut m);
                Arc::new(m)
            },
            spawn_func: spawn_div,
        },
        NativeWidget {
            name: String::from("img"),
            default_properties: {
                let mut m = HashMap::new();
                node_properties(&mut m);
                background_color_properties(&mut m);
                border_color_properties(&mut m);
                border_radius_properties(&mut m);
                image_properties(&mut m);
                Arc::new(m)
            },
            spawn_func: spawn_img,
        },
        NativeWidget {
            name: String::from("p"),
            default_properties: {
                let mut m = HashMap::new();
                node_properties(&mut m);
                background_color_properties(&mut m);
                border_color_properties(&mut m);
                border_radius_properties(&mut m);
                text_properties(&mut m);
                Arc::new(m)
            },
            spawn_func: spawn_p,
        },
        NativeWidget {
            name: String::from("span"),
            default_properties: {
                let mut m = HashMap::new();
                node_properties(&mut m);
                background_color_properties(&mut m);
                border_color_properties(&mut m);
                border_radius_properties(&mut m);
                text_span_properties(&mut m);
                Arc::new(m)
            },
            spawn_func: spawn_span,
        }
    ];
}

/// Inserts the default properties for a [`Node`] into the given map.
fn node_properties(m: &mut HashMap<String, PropertyValue>) {
    m.insert("display".into(), "flex".into());
    m.insert("box-sizing".into(), "border-box".into());
    m.insert("position-type".into(), "relative".into());

    m.insert("overflow-x".into(), "visible".into());
    m.insert("overflow-y".into(), "visible".into());
    m.insert("scrollbar-width".into(), 0.into());

    m.insert("overflow-clip-margin-box".into(), "padding-box".into());
    m.insert("overflow-clip-margin".into(), 0.into());

    m.insert("left".into(), "auto".into());
    m.insert("top".into(), "auto".into());
    m.insert("right".into(), "auto".into());
    m.insert("bottom".into(), "auto".into());

    m.insert("width".into(), "auto".into());
    m.insert("height".into(), "auto".into());
    m.insert("min-width".into(), "auto".into());
    m.insert("min-height".into(), "auto".into());
    m.insert("max-width".into(), "auto".into());
    m.insert("max-height".into(), "auto".into());
    m.insert("aspect-ratio".into(), "none".into());

    m.insert("align-items".into(), "default".into());
    m.insert("justify-items".into(), "default".into());
    m.insert("align-self".into(), "auto".into());
    m.insert("justify-self".into(), "auto".into());
    m.insert("align-content".into(), "default".into());
    m.insert("justify-content".into(), "default".into());

    m.insert("margin".into(), 0.into());
    m.insert("margin-top".into(), 0.into());
    m.insert("margin-left".into(), 0.into());
    m.insert("margin-right".into(), 0.into());
    m.insert("margin-bottom".into(), 0.into());

    m.insert("padding".into(), 0.into());
    m.insert("padding-top".into(), 0.into());
    m.insert("padding-left".into(), 0.into());
    m.insert("padding-right".into(), 0.into());
    m.insert("padding-bottom".into(), 0.into());

    m.insert("border-thickness".into(), 0.into());
    m.insert("border-thickness-top".into(), 0.into());
    m.insert("border-thickness-left".into(), 0.into());
    m.insert("border-thickness-right".into(), 0.into());
    m.insert("border-thickness-bottom".into(), 0.into());

    m.insert("flex-direction".into(), "row".into());
    m.insert("flex-wrap".into(), "nowrap".into());
    m.insert("flex-grow".into(), 0.into());
    m.insert("flex-shrink".into(), 1.into());
    m.insert("flex-basis".into(), "auto".into());

    m.insert("row-gap".into(), 0.into());
    m.insert("column-gap".into(), 0.into());

    m.insert("grid-auto-flow".into(), "row".into());
    // m.insert("grid-template-rows".into(), "none".into());
    // m.insert("grid-template-columns".into(), "none".into());
    // m.insert("grid-auto-rows".into(), "auto".into());
    // m.insert("grid-auto-columns".into(), "auto".into());
    // m.insert("grid-row".into(), "auto".into());
    // m.insert("grid-column".into(), "auto".into());
}

/// Inserts the default properties for a [`BackgroundColor`] into the given map.
fn background_color_properties(m: &mut HashMap<String, PropertyValue>) {
    m.insert("background-color".into(), Color::NONE.into());
}

/// Inserts the default properties for a [`BorderColor`] into the given map.
fn border_color_properties(m: &mut HashMap<String, PropertyValue>) {
    m.insert("border-color".into(), Color::NONE.into());
    m.insert("border-color-top".into(), Color::NONE.into());
    m.insert("border-color-left".into(), Color::NONE.into());
    m.insert("border-color-right".into(), Color::NONE.into());
    m.insert("border-color-bottom".into(), Color::NONE.into());
}

/// Inserts the default properties for a [`BorderRadius`] into the given map.
fn border_radius_properties(m: &mut HashMap<String, PropertyValue>) {
    m.insert("border-radius".into(), 0.into());
    m.insert("border-radius-top-left".into(), 0.into());
    m.insert("border-radius-top-right".into(), 0.into());
    m.insert("border-radius-bottom-left".into(), 0.into());
    m.insert("border-radius-bottom-right".into(), 0.into());
}

/// Inserts the default properties for an [`ImageNode`] into the given map.
fn image_properties(m: &mut HashMap<String, PropertyValue>) {
    m.insert("src".into(), "".into());
    m.insert("tint".into(), Color::WHITE.into());
    m.insert("flip-x".into(), false.into());
    m.insert("flip-y".into(), false.into());
    m.insert("mode".into(), "auto".into());

    // slice mode properties
    m.insert("slice-size".into(), 0.into());
    m.insert("slice-size-top".into(), 0.into());
    m.insert("slice-size-right".into(), 0.into());
    m.insert("slice-size-bottom".into(), 0.into());
    m.insert("slice-size-left".into(), 0.into());

    m.insert("max-corner-scale".into(), 1.into());
    m.insert("center-scale-mode".into(), "stretch".into());
    m.insert("center-scale-stretch".into(), 1.into());
    m.insert("sides-scale-mode".into(), "stretch".into());
    m.insert("sides-scale-stretch".into(), 1.into());

    // tile mode properties
    m.insert("tile-x".into(), true.into());
    m.insert("tile-y".into(), true.into());
    m.insert("stretch-value".into(), 1.into());
}

/// Inserts the default properties for a [`Text`] bundle into the given map.
fn text_properties(m: &mut HashMap<String, PropertyValue>) {
    // Text
    m.insert("text".into(), "".into());

    // TextFont
    m.insert("font".into(), "auto".into());
    m.insert("font-size".into(), 16.into());
    m.insert("line-height".into(), PropertyValue::Percent(120.0));
    m.insert("font-smoothing".into(), "antialiased".into());

    // TextLayout
    m.insert("justify".into(), "left".into());
    m.insert("line-break".into(), "word".into());

    // TextColor
    m.insert("color".into(), Color::WHITE.into());
}

/// Inserts the default properties for a [`TextSpan`] bundle into the given map.
fn text_span_properties(m: &mut HashMap<String, PropertyValue>) {
    // TextSpan
    m.insert("text".into(), "".into());

    // TextFont
    m.insert("font".into(), "auto".into());
    m.insert("font-size".into(), 16.into());
    m.insert("line-height".into(), PropertyValue::Percent(120.0));
    m.insert("font-smoothing".into(), "antialiased".into());

    // TextColor
    m.insert("color".into(), Color::WHITE.into());
}

/// Spawns a `div` native widget.
fn spawn_div(
    _: &Res<AssetServer>,
    commands: &mut Commands,
    element: &NekoElement,
    parent: Entity,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            node_bundle(element),
            background_color_bundle(element),
            border_color_bundle(element),
            border_radius_bundle(element),
        ))
        .id()
}

/// Spawns an `img` native widget.
fn spawn_img(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    element: &NekoElement,
    parent: Entity,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            node_bundle(element),
            background_color_bundle(element),
            border_color_bundle(element),
            border_radius_bundle(element),
            image_node_bundle(asset_server, element),
        ))
        .id()
}

/// Spawns an `p` native widget.
fn spawn_p(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    element: &NekoElement,
    parent: Entity,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            node_bundle(element),
            background_color_bundle(element),
            border_color_bundle(element),
            border_radius_bundle(element),
            text_node_bundle(asset_server, element),
        ))
        .id()
}

/// Spawns an `span` native widget.
fn spawn_span(
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    element: &NekoElement,
    parent: Entity,
) -> Entity {
    commands
        .spawn((
            ChildOf(parent),
            node_bundle(element),
            background_color_bundle(element),
            border_color_bundle(element),
            border_radius_bundle(element),
            span_node_bundle(asset_server, element),
        ))
        .id()
}

/// Build [`Node`] bundle
fn node_bundle(element: &NekoElement) -> Node {
    let border_thickness = element.get_as("border-thickness");
    let margin = element.get_as("margin");
    let padding = element.get_as("padding");

    Node {
        display: element.get_as("display"),
        box_sizing: element.get_as("box-sizing"),
        position_type: element.get_as("position-type"),

        overflow: Overflow {
            x: element.get_as("overflow-x"),
            y: element.get_as("overflow-y"),
        },
        scrollbar_width: element.get_as("scrollbar-width"),
        overflow_clip_margin: OverflowClipMargin {
            visual_box: element.get_as("overflow-clip-margin-box"),
            margin: element.get_as("overflow-clip-margin"),
        },

        left: element.get_as("left"),
        top: element.get_as("top"),
        right: element.get_as("right"),
        bottom: element.get_as("bottom"),

        width: element.get_as("width"),
        height: element.get_as("height"),
        min_width: element.get_as("min-width"),
        min_height: element.get_as("min-height"),
        max_width: element.get_as("max-width"),
        max_height: element.get_as("max-height"),
        aspect_ratio: element.get_as("aspect-ratio"),

        align_items: element.get_as("align-items"),
        justify_items: element.get_as("justify-items"),
        align_self: element.get_as("align-self"),
        justify_self: element.get_as("justify-self"),
        align_content: element.get_as("align-content"),
        justify_content: element.get_as("justify-content"),

        margin: UiRect {
            top: element.get_no_default("margin-top", margin),
            left: element.get_no_default("margin-left", margin),
            right: element.get_no_default("margin-right", margin),
            bottom: element.get_no_default("margin-bottom", margin),
        },

        padding: UiRect {
            top: element.get_no_default("padding-top", padding),
            left: element.get_no_default("padding-left", padding),
            right: element.get_no_default("padding-right", padding),
            bottom: element.get_no_default("padding-bottom", padding),
        },

        border: UiRect {
            top: element.get_no_default("border-thickness-top", border_thickness),
            left: element.get_no_default("border-thickness-left", border_thickness),
            right: element.get_no_default("border-thickness-right", border_thickness),
            bottom: element.get_no_default("border-thickness-bottom", border_thickness),
        },

        flex_direction: element.get_as("flex-direction"),
        flex_wrap: element.get_as("flex-wrap"),
        flex_grow: element.get_as("flex-grow"),
        flex_shrink: element.get_as("flex-shrink"),
        flex_basis: element.get_as("flex-basis"),

        row_gap: element.get_as("row-gap"),
        column_gap: element.get_as("column-gap"),

        grid_auto_flow: element.get_as("grid-auto-flow"),
        // grid_template_rows: element.get_as("grid-template-rows"),
        // grid_template_columns: element.get_as("grid-template-columns"),
        // grid_auto_rows: element.get_as("grid-auto-rows"),
        // grid_auto_columns: element.get_as("grid-auto-columns"),
        // grid_row: element.get_as("grid-row"),
        // grid_column: element.get_as("grid-column"),
        ..default()
    }
}

/// Build [`BorderColor`] bundle
fn border_color_bundle(element: &NekoElement) -> BorderColor {
    let border_color = element.get_as("border-color");

    BorderColor {
        top: element.get_no_default("border-color-top", border_color),
        left: element.get_no_default("border-color-left", border_color),
        right: element.get_no_default("border-color-right", border_color),
        bottom: element.get_no_default("border-color-bottom", border_color),
    }
}

/// Build [`BackgroundColor`] bundle
fn background_color_bundle(element: &NekoElement) -> BackgroundColor {
    BackgroundColor(element.get_as("background-color"))
}

/// Build [`BorderRadius`] bundle
fn border_radius_bundle(element: &NekoElement) -> BorderRadius {
    let border_radius = element.get_as("border-radius");

    BorderRadius {
        top_left: element.get_no_default("border-radius-top-left", border_radius),
        top_right: element.get_no_default("border-radius-top-right", border_radius),
        bottom_left: element.get_no_default("border-radius-bottom-left", border_radius),
        bottom_right: element.get_no_default("border-radius-bottom-right", border_radius),
    }
}

/// Build [`ImageNode`] bundle
fn image_node_bundle(asset_server: &Res<AssetServer>, element: &NekoElement) -> ImageNode {
    let src: String = element.get_as("src");
    let slice_size = element.get_as("slice-size");

    ImageNode {
        color: element.get_as("tint"),
        image: asset_server.load(src),
        flip_x: element.get_as("flip-x"),
        flip_y: element.get_as("flip-y"),
        image_mode: match element.get_property("mode") {
            Some(PropertyValue::String(s)) if s == "auto" => NodeImageMode::Auto,
            Some(PropertyValue::String(s)) if s == "stretch" => NodeImageMode::Stretch,
            Some(PropertyValue::String(s)) if s == "sliced" => {
                NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect {
                        top: element.get_no_default("slice-size-top", slice_size),
                        left: element.get_no_default("slice-size-left", slice_size),
                        right: element.get_no_default("slice-size-right", slice_size),
                        bottom: element.get_no_default("slice-size-bottom", slice_size),
                    },
                    center_scale_mode: match element.get_property("center-scale-mode") {
                        Some(PropertyValue::String(s)) if s == "stretch" => SliceScaleMode::Stretch,
                        Some(PropertyValue::String(s)) if s == "tile" => SliceScaleMode::Tile {
                            stretch_value: element.get_as("center-scale-stretch"),
                        },
                        Some(property) => {
                            warn!(
                                "Failed to convert PropertyValue {} to SliceScaleMode",
                                property
                            );
                            SliceScaleMode::default()
                        }
                        None => SliceScaleMode::default(),
                    },
                    sides_scale_mode: match element.get_property("sides-scale-mode") {
                        Some(PropertyValue::String(s)) if s == "stretch" => SliceScaleMode::Stretch,
                        Some(PropertyValue::String(s)) if s == "tile" => SliceScaleMode::Tile {
                            stretch_value: element.get_as("sides-scale-stretch"),
                        },
                        Some(property) => {
                            warn!(
                                "Failed to convert PropertyValue {} to SliceScaleMode",
                                property
                            );
                            SliceScaleMode::default()
                        }
                        None => SliceScaleMode::default(),
                    },
                    max_corner_scale: element.get_as("max-corner-scale"),
                })
            }
            Some(PropertyValue::String(s)) if s == "tiled" => NodeImageMode::Tiled {
                tile_x: element.get_as("tile-x"),
                tile_y: element.get_as("tile-y"),
                stretch_value: element.get_as("stretch-value"),
            },
            Some(property) => {
                warn!(
                    "Failed to convert PropertyValue {} to NodeImageMode",
                    property
                );
                NodeImageMode::default()
            }
            None => NodeImageMode::default(),
        },
        ..default()
    }
}

/// Build [`Text`] bundle
fn text_node_bundle(asset_server: &Res<AssetServer>, element: &NekoElement) -> impl Bundle {
    let font: String = element.get_as("font");

    (
        Text(element.get_as("text")),
        TextFont {
            font: match font {
                s if s == "auto" => Handle::<Font>::default(),
                font_path => asset_server.load(font_path),
            },
            font_size: element.get_as("font-size"),
            line_height: element.get_as("line-height"),
            font_smoothing: element.get_as("font-smoothing"),
        },
        TextLayout {
            justify: element.get_as("justify"),
            linebreak: element.get_as("line-break"),
        },
        TextColor(element.get_as("color")),
    )
}

/// Build [`TextSpan`] bundle
fn span_node_bundle(asset_server: &Res<AssetServer>, element: &NekoElement) -> impl Bundle {
    let font: String = element.get_as("font");

    (
        TextSpan(element.get_as("text")),
        TextFont {
            font: match font {
                s if s == "auto" => Handle::<Font>::default(),
                font_path => asset_server.load(font_path),
            },
            font_size: element.get_as("font-size"),
            line_height: element.get_as("line-height"),
            font_smoothing: element.get_as("font-smoothing"),
        },
        TextColor(element.get_as("color")),
    )
}
