use snafu::Snafu;

use crate::map_generators;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(context(false))]
    #[snafu(display("Error in map_generators: {:?}", source.to_string()))]
    MapGeneratorError { source: map_generators::Error },
}
