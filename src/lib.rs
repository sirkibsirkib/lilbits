extern crate serde;
#[macro_use] extern crate serde_derive;

#[macro_use]
mod lilbitset;
#[cfg(test)]
mod tests;


pub use lilbitset::{LilBitSet,IntoIter};