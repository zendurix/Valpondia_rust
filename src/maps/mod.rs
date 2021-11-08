pub mod corridor;
pub(crate) mod errors;
pub mod generators;
pub mod map;
pub mod rect;

pub use errors::Error;

use errors::Result;
use rltk::Point;

pub use self::map::{Map, TileType};

pub trait MapGenerator {
    fn generate(self, prev_down_stairs_pos: Option<Point>) -> Result<Map>;
}
