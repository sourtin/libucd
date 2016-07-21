#![no_std]

pub mod tables;
pub mod getters;
pub struct Codepoint(pub char);

pub use tables::general::{UnicodeBlock, UnicodeCategory};
pub use tables::bidi::{BidiClass, BidiPairedBracketType};
