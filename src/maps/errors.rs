use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display(
        "Wrong map dimensions (can't create map with one dimension being 0). Map dimensions: ({}, {})", 
        map_dimensions.0,
        map_dimensions.1
    ))]
    IncorrectMapDimensions { map_dimensions: (usize, usize) },

    #[snafu(display("Too many tries to split area in BSP.",))]
    TooManyBSPSplitRetries,

    #[snafu(display("Area of BSP node is to small to split into two areas.",))]
    TooSmallBSPAreaToSplit,
}
