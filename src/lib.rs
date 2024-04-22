#![no_std]
#![feature(const_mut_refs)]
#![feature(const_option)]

pub mod color;
pub mod date;
pub mod fractal;
pub mod instab;
pub mod opt;
pub mod parse;
pub mod rating;
pub mod set;

pub use date::Date;
pub use fractal::Fractal;
pub use instab::Instability;
pub use opt::Opts;
pub use parse::Parsable;
pub use rating::{Boss, Rateable, Rater, Rating};
pub use set::{HasInstabs, Searchable, Set};
