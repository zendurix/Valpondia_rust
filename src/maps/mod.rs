pub(crate) mod errors;
pub mod generators;
pub mod map;

pub use errors::Error;

use errors::Result;

pub use self::map::{Map, TileType};

pub trait MapGenerator {
    fn generate(self) -> Result<Map>;
}
