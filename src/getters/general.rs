use ::tables::general::*;
use ::getters::UCDSearch;

impl ::Codepoint {
    pub fn age(self) -> Option<(u8,u8)> { UCD_AGE.search(self.0) }
}
