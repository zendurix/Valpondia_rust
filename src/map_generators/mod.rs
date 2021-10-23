use crate::data::map::Map;

pub mod cellular_automata;

pub trait MapGenerator {
    fn generate(self) -> Map;
}
