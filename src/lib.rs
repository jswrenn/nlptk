#![feature(conservative_impl_trait)]
#![feature(try_from)]
#![feature(non_ascii_idents)]
#![allow(non_snake_case)]
#![warn(missing_docs)]
extern crate num;
extern crate itertools;

#[macro_use]
mod language;
pub use language::*;

mod token;
pub use token::*;

mod corpus;
pub use corpus::*;
