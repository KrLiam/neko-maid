#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use bevy::prelude::*;

pub mod parse;
pub mod vm;

/// A Bevy UI plugin: NekoMaid
///
/// This plugin provides core functionality for the NekoMaid framework,
/// including UI components and systems, assets, and high-level widgets.
pub struct NekoMaidPlugin;
impl Plugin for NekoMaidPlugin {
    fn build(&self, _: &mut App) {}
}
