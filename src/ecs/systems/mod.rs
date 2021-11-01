mod movement;
mod update_view;

pub(crate) mod ai;
pub mod player;

pub use ai::ai_main;
pub use movement::move_all;
pub use update_view::{update_view, update_view_memory};
