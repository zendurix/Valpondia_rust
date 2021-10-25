use snafu::Snafu;

use crate::levels;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(context(false))]
    #[snafu(display("Error in levels: {:?}", source.to_string()))]
    LevelError { source: levels::Error },
}
