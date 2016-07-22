use ::tables::*;
use core::cmp::Ordering::{Equal, Less, Greater};
use core::char;

fn search_range<S>(table: &[((u8,u8,u8),(u8,u8,u8),S)], cp: char) -> Option<S>
    where S: Clone
{
    let cp = cp as u32;
    match table.binary_search_by(|&((rb1,rb2,rb3), (re1,re2,re3), _)| {
        let rb: u32 = (rb1 as u32)*65536 + (rb2 as u32)*256 + (rb3 as u32);
        let re: u32 = (re1 as u32)*65536 + (re2 as u32)*256 + (re3 as u32);
        if rb <= cp && cp <= re { Equal }
        else if re < cp { Less }
        else { Greater }
    }) {
        Ok(idx) => {
            let (_, _, ref v) = table[idx];
            Some(v.clone())
        },
        _ => None
    }
}

fn search<S>(table: &[((u8,u8,u8),S)], cp: char) -> Option<S>
    where S: Clone
{
    let ca = cp as u32;
    match table.binary_search_by(|&((cb1,cb2,cb3), _)| {
        let cb: u32 = (cb1 as u32)*65536 + (cb2 as u32)*256 + (cb3 as u32);
        cb.cmp(&ca)
    }) {
        Ok(idx) => {
            let (_, ref v) = table[idx];
            Some(v.clone())
        },
        _ => None
    }
}

fn in_ranges(table: &[((u8,u8,u8),(u8,u8,u8),())], cp: char) -> bool {
    match search_range(table, cp) { Some(()) => true, None => false } }
fn in_table(table: &[((u8,u8,u8),())], cp: char) -> bool {
    match search(table, cp) { Some(()) => true, None => false } }

fn map16(table: &[(u16,u16)], cp: char) -> Option<char> {
    let ca = cp as u32;
    if ca > 65536 { return None; }
    let cb = ca as u16;

    match table.binary_search_by(|&(cc,_)| cc.cmp(&cb)) {
        Ok(idx) => {
            let (_, v) = table[idx];
            char::from_u32(v as u32)
        },
        _ => None
    }
}

#[derive(Clone,Copy,Eq,PartialEq,Debug,Ord,PartialOrd)]
pub enum Number {
    Integer(i64),
    Rational(i32,u32)
}

impl ::Codepoint {
    // general
    pub fn age(self) -> Option<(u8,u8)> { search_range(&UCD_AGE, self.0) }
    pub fn block(self) -> Option<UnicodeBlock> { search_range(&UCD_BLOCK, self.0) }
    pub fn category(self) -> UnicodeCategory {
        search_range(&UCD_CAT, self.0).unwrap_or(UnicodeCategory::Unassigned) }
    pub fn combining_class(self) -> u8 { search_range(&UCD_COMBCLS, self.0).unwrap_or(0) }
    pub fn iso_comment(self) -> &'static str { "" }

    // bidi
    pub fn bidi_control(self) -> bool {
        match self.0 as u32 {
            1564 | 8206 | 8207 | 8234...8238 | 8294...8297 => true,
            _ => false
        }
    }
    pub fn bidi_class(self) -> BidiClass {
        search_range(&UCD_BIDI_CLASS, self.0).unwrap_or(BidiClass::LeftToRight) }
    pub fn bidi_mirrored(self) -> bool { in_ranges(&UCD_BIDI_MIRRORED, self.0) }
    pub fn bidi_paired_bracket_type(self) -> Option<BidiPairedBracketType> {
        search_range(&UCD_BIDI_BRATYPE, self.0) }
    pub fn bidi_mirror(self) -> Option<char> { map16(&UCD_BIDI_MIRROR, self.0) }
    pub fn bidi_paired_bracket(self) -> char { map16(&UCD_BIDI_PAIRED, self.0).unwrap_or(self.0) }

    // misc
    pub fn east_asian_width(self) -> EastAsianWidth {
        search_range(&UCD_EAWIDTH, self.0).unwrap_or(EastAsianWidth::Neutral) }
    pub fn linebreak_class(self) -> Option<LinebreakClass> { search_range(&UCD_LB, self.0) }
    pub fn numeric_type(self) -> Option<NumericType> { search_range(&UCD_NUMTYPE, self.0) }
    pub fn numeric_value(self) -> Option<Number> {
        search(&UCD_NUMVAL, self.0).map(|i| {
            match UCD_NUMS[i as usize] {
                (num, 1) => Number::Integer(num),
                (num, den) => Number::Rational(num as i32, den as u32)
            }
        })
    }
    pub fn deprecated(self) -> bool {
        match self.0 as u32 {
            329 | 1651 | 3959 | 3961 | 6051 | 6052
                | 8298...8303 | 9001 | 9002 | 917505 => true,
            _ => false
        }
    }
    pub fn variation_selector(self) -> bool {
        let cp = self.0 as u32;
        (cp >= 917760 && cp <= 917999)   ||
            (cp >= 65024 && cp <= 65039) ||
            (cp >= 6155 && cp <= 6157)
    }
    pub fn noncharacter(self) -> bool {
        let cp = self.0 as u32;
        (cp >= 0xfdd0 && cp <= 0xfdef) ||
            ((cp & 0xffff) >= 0xfffe)
    }

    // scripts
    // > arabic
    pub fn join_control(self) -> bool { let cp = self.0 as u32; cp == 8204 || cp == 8205 }
    pub fn joining_group(self) -> JoiningGroup {
        search(&UCD_JOINGRP, self.0).unwrap_or(JoiningGroup::NoJoiningGroup) }
    pub fn joining_type(self) -> JoiningType {
        search_range(&UCD_JOINTYPE, self.0).unwrap_or(JoiningType::NonJoining) }

    // function and graphic characteristics
    pub fn is_ascii_hex_digit(self) -> bool { in_table(&UCD_HEX_DIGIT_ASCII, self.0) }
    pub fn is_preprended_concatenation_mark(self) -> bool { in_table(&UCD_PREPENDED_CONCATENATION_MARK, self.0) }
    pub fn is_hyphen(self) -> bool { in_table(&UCD_HYPHEN, self.0) }
    pub fn is_hex_digit(self) -> bool { in_table(&UCD_HEX_DIGIT, self.0) }
    pub fn is_whitespace(self) -> bool { in_table(&UCD_WHITE, self.0) }
    pub fn is_logical_order_exception(self) -> bool { in_table(&UCD_LOGICAL_ORDER_EXCEPTION, self.0) }
    pub fn is_sentence_terminal(self) -> bool { in_table(&UCD_TERM_SENTENCE, self.0) }
    pub fn is_dash(self) -> bool { in_table(&UCD_DASH, self.0) }
    pub fn is_quotation_mark(self) -> bool { in_table(&UCD_QUOT, self.0) }
    pub fn is_terminal_punctutation(self) -> bool { in_table(&UCD_TERM_PUNC, self.0) }
    pub fn is_extender(self) -> bool { in_table(&UCD_EXTENDER, self.0) }
    pub fn is_soft_dotted(self) -> bool { in_table(&UCD_SOFT_DOTTED, self.0) }
    pub fn is_default_ignorable(self) -> bool { in_ranges(&UCD_DEFAULT_IGNORABLE, self.0) }
    pub fn is_alphabetic(self) -> bool { in_ranges(&UCD_ALPHA, self.0) }
    pub fn is_default_ignorable_other(self) -> bool { in_ranges(&UCD_DEFAULT_IGNORABLE_OTHER, self.0) }
    pub fn is_math_other(self) -> bool { in_ranges(&UCD_MATH_OTHER, self.0) }
    pub fn is_diacritic(self) -> bool { in_ranges(&UCD_DIACRITIC, self.0) }
    pub fn is_math(self) -> bool { in_ranges(&UCD_MATH, self.0) }
    pub fn is_alphabetic_other(self) -> bool { in_ranges(&UCD_ALPHA_OTHER, self.0) }
}
