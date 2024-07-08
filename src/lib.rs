#![allow(clippy::needless_borrows_for_generic_args)]
pub use byteordered::Endianness;
mod botw;
// mod cli;
pub mod model;
// mod subcommand;
mod util;

pub use crate::model::Msyt;
pub type Result<T> = std::result::Result<T, anyhow::Error>;
