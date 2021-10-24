use crate::levels::map::Map;

pub mod cellular_automata;
pub(crate) mod errors;

pub use errors::Error;

use errors::Result;

pub trait MapGenerator {
    fn generate(self) -> Result<Map>;
}
