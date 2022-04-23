mod display;
mod solver;
use ndarray::Array2;
pub use serde_derive::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FlowFreeBoard {
    /// - `-1` : cap = 1
    /// - `0` : cap = 0
    board: Array2<i8>,
}
