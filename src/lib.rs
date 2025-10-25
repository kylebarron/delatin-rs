// pub mod delatin;
pub mod error;
mod queue;
mod triangulation;
mod utils;

#[cfg(test)]
mod tests;

pub use crate::triangulation::Triangulation;
