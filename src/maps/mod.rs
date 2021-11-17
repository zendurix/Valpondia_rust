pub mod errors;
pub mod generators;
pub mod map;
pub mod rect;

pub use errors::Error;

use errors::Result;
use rltk::Point;

pub use self::map::{Map, TileType};
