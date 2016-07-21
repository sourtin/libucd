use ::tables::general::*;
use ::getters::{search, search_range};

impl ::Codepoint {
    pub fn age(self) -> Option<(u8,u8)> { search_range(&UCD_AGE, self.0) }
    pub fn block(self) -> Option<UnicodeBlock> { search_range(&UCD_BLOCK, self.0) }
    pub fn category(self) -> UnicodeCategory {
        search_range(&UCD_CAT, self.0).unwrap_or(UnicodeCategory::Unassigned) }
    pub fn combining_class(self) -> u8 { search_range(&UCD_COMBCLS, self.0).unwrap_or(0) }
}
