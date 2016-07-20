use ::tables::general::*;
use ::getters::{search, search_range};

impl ::Codepoint {
    pub fn age(self) -> Option<(u8,u8)> { search_range(&UCD_AGE, self.0) }
}
