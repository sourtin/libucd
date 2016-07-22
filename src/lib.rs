#![no_std]

pub mod tables;
pub mod getters;
pub struct Codepoint(pub char);

pub use tables::general::{UnicodeBlock, UnicodeCategory};
pub use tables::bidi::{BidiClass, BidiPairedBracketType};
pub use tables::misc::{NumericType, EastAsianWidth, LinebreakClass};
pub use tables::scripts::{JoiningGroup, JoiningType};

pub use getters::Number;
pub use getters::CharIter;
