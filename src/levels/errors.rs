use snafu::Snafu;

use crate::maps;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(context(false))]
    #[snafu(display("Error in maps: {:?}", source.to_string()))]
    MapDError { source: maps::Error },
}
