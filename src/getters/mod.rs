use ::tables::*;
use core::cmp::Ordering::{Equal, Less, Greater};
use core::char;
use core::slice;

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

fn cp_decode((c1,c2,c3): (u8,u8,u8)) -> char {
    let c = (c1 as u32)*65536 + (c2 as u32)*256 + (c3 as u32);
    unsafe { char::from_u32_unchecked(c) }
}

enum CharIterInternal {
    Iterator(slice::Iter<'static, (u8,u8,u8)>),
    Single(char),
    Exhausted
}

pub struct CharIter(CharIterInternal);

impl CharIter {
    pub fn new(osl: Option<&'static [(u8,u8,u8)]>, cp: ::Codepoint) -> CharIter {
        CharIter(match osl {
            Some(sl) => CharIterInternal::Iterator(sl.iter()),
            None => CharIterInternal::Single(cp.codepoint())
        })
    }
}

impl Iterator for CharIter {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.0 {
            CharIterInternal::Iterator(ref mut it) => it.next().map(|c| cp_decode(*c)),
            CharIterInternal::Single(c) => {
                self.0 = CharIterInternal::Exhausted;
                Some(c)
            },
            CharIterInternal::Exhausted => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.0 {
            CharIterInternal::Iterator(ref it) => it.size_hint(),
            CharIterInternal::Single(_) => (1, Some(1)),
            CharIterInternal::Exhausted => (0, Some(0))
        }
    }
}

#[derive(Clone,Copy,Eq,PartialEq,Debug,Ord,PartialOrd)]
pub enum Number {
    Integer(i64),
    Rational(i32,u32)
}

impl ::Codepoint {
    // general
    pub fn codepoint(self) -> char { self.0 }
    pub fn age(self) -> Option<(u8,u8)> { search_range(&UCD_AGE, self.0) }
    pub fn block(self) -> Option<UnicodeBlock> { search_range(&UCD_BLOCK, self.0) }
    pub fn category(self) -> UnicodeCategory {
        search_range(&UCD_CAT, self.0).unwrap_or(UnicodeCategory::Unassigned) }
    pub fn combining_class(self) -> u8 { search_range(&UCD_COMBCLS, self.0).unwrap_or(0) }
    pub fn iso_comment(self) -> &'static str { "" }

    // bidi
    pub fn bidi_is_control(self) -> bool {
        match self.0 as u32 {
            1564 | 8206 | 8207 | 8234...8238 | 8294...8297 => true,
            _ => false
        }
    }
    pub fn bidi_class(self) -> BidiClass {
        search_range(&UCD_BIDI_CLASS, self.0).unwrap_or(BidiClass::LeftToRight) }
    pub fn bidi_is_mirrored(self) -> bool { in_ranges(&UCD_BIDI_MIRRORED, self.0) }
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
    pub fn is_deprecated(self) -> bool {
        match self.0 as u32 {
            329 | 1651 | 3959 | 3961 | 6051 | 6052
                | 8298...8303 | 9001 | 9002 | 917505 => true,
            _ => false
        }
    }
    pub fn is_variation_selector(self) -> bool {
        let cp = self.0 as u32;
        (cp >= 917760 && cp <= 917999)   ||
            (cp >= 65024 && cp <= 65039) ||
            (cp >= 6155 && cp <= 6157)
    }
    pub fn is_noncharacter(self) -> bool {
        let cp = self.0 as u32;
        (cp >= 0xfdd0 && cp <= 0xfdef) ||
            ((cp & 0xffff) >= 0xfffe)
    }

    // scripts
    pub fn script(self) -> Option<Script> { search_range(&UCD_SCRIPT, self.0) }
    pub fn script_extensions(self) -> Option<&'static [Script]> {
        match search(&UCD_SCRIPTEXT, self.0) {
            None => self.script().map(|s| UCD_SCRIPT_MAP[s as usize]),
            x => x
        }
    }
    // > arabic
    pub fn join_control(self) -> bool { let cp = self.0 as u32; cp == 8204 || cp == 8205 }
    pub fn joining_group(self) -> JoiningGroup {
        search(&UCD_JOINGRP, self.0).unwrap_or(JoiningGroup::NoJoiningGroup) }
    pub fn joining_type(self) -> JoiningType {
        search_range(&UCD_JOINTYPE, self.0).unwrap_or(JoiningType::NonJoining) }
    // > indic
    pub fn indic_syllabic_category(self) -> IndicSyllabicCategory {
        search_range(&UCD_INSC, self.0).unwrap_or(IndicSyllabicCategory::Other) }
    pub fn indic_positional_category(self) -> Option<IndicPositionalCategory> { search(&UCD_INPC, self.0) }
    // > hangul
    pub fn jamo_short_name(self) -> Option<&'static str> { search(&UCD_JSN, self.0) }
    pub fn hangul_syllable_type(self) -> Option<HangulSyllableType> {
        let cp = self.0 as u32;
        match cp {
            4352...4447 | 43360...43388 => Some(HangulSyllableType::LeadingJamo),
            4448...4519 | 55216...55238 => Some(HangulSyllableType::VowelJamo),
            4520...4607 | 55243...55291 => Some(HangulSyllableType::TrailingJamo),
            44032...55203 => Some({
                if cp % 28 == 16 { HangulSyllableType::LVSyllable }
                else {HangulSyllableType::LVTSyllable }
            }),
            _ => None
        }
    }

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

    // remaining bools
    pub fn changes_when_casefolded(self) -> bool { in_table(&UCD_CASE_CHANGES_CASEFOLD, self.0) }
    pub fn changes_when_casefolded_nfkc(self) -> bool { in_ranges(&UCD_CASE_CHANGES_CASEFOLD_NFKC, self.0) }
    pub fn changes_when_casemapped(self) -> bool { in_ranges(&UCD_CASE_CHANGES_CASEMAP, self.0) }
    pub fn changes_when_lowercased(self) -> bool { in_table(&UCD_CASE_CHANGES_LOWER, self.0) }
    pub fn changes_when_titlecased(self) -> bool { in_table(&UCD_CASE_CHANGES_TITLE, self.0) }
    pub fn changes_when_uppercased(self) -> bool { in_table(&UCD_CASE_CHANGES_UPPER, self.0) }
    pub fn excluded_from_composition(self) -> bool { in_table(&UCD_COMP_EXCL, self.0) }
    pub fn excluded_from_composition_fully(self) -> bool { in_ranges(&UCD_COMP_EXCL_FULL, self.0) }
    pub fn expands_on_nfc(self) -> bool { in_table(&UCD_EXPANDING_NFC, self.0) }
    pub fn expands_on_nfd(self) -> bool { in_ranges(&UCD_EXPANDING_NFD, self.0) }
    pub fn expands_on_nfkc(self) -> bool { in_ranges(&UCD_EXPANDING_NFKC, self.0) }
    pub fn expands_on_nfkd(self) -> bool { in_ranges(&UCD_EXPANDING_NFKD, self.0) }
    pub fn is_case_ignorable(self) -> bool { in_ranges(&UCD_CASE_IGNORABLE, self.0) }
    pub fn is_cased(self) -> bool { in_ranges(&UCD_CASED, self.0) }
    pub fn is_grapheme_base(self) -> bool { in_ranges(&UCD_GRAPH_BASE, self.0) }
    pub fn is_grapheme_extend(self) -> bool { in_ranges(&UCD_GRAPH_EXT, self.0) }
    pub fn is_grapheme_extend_other(self) -> bool { in_ranges(&UCD_GRAPH_EXT_OTHER, self.0) }
    pub fn is_grapheme_link(self) -> bool { in_table(&UCD_GRAPH_LINK, self.0) }
    pub fn is_id_continue(self) -> bool { in_ranges(&UCD_ID_CONT, self.0) }
    pub fn is_id_continue_nfkc(self) -> bool { in_ranges(&UCD_ID_CONT_NFKC, self.0) }
    pub fn is_id_continue_other(self) -> bool { 
        match self.0 as u32 {
            183 | 903 | 4969...4977 | 6618 => true,
            _ => false } }
    pub fn is_id_start(self) -> bool { in_ranges(&UCD_ID_START, self.0) }
    pub fn is_id_start_nfkc(self) -> bool { in_ranges(&UCD_ID_START_NFKC, self.0) }
    pub fn is_id_start_other(self) -> bool {
        match self.0 as u32 {
            6277 | 6278 | 8472 | 8494 | 12443 | 12444 => true,
            _ => false } }
    pub fn is_ideograph(self) -> bool { in_ranges(&UCD_IDEO, self.0) }
    pub fn is_ideograph_description_sequence_binary_operator(self) -> bool {
        match self.0 as u32 {
            12272 | 12273 | 12276...12283 => true,
            _ => false } }
    pub fn is_ideograph_description_sequence_radical(self) -> bool {
        match self.0 as u32 {
            11904...11929 | 11931...12019 | 12032...12245 => true,
            _ => false } }
    pub fn is_ideograph_description_sequence_trinary_operator(self) -> bool { let cp = self.0 as u32; cp == 12274 || cp == 12275 }
    pub fn is_ideograph_unified(self) -> bool { in_ranges(&UCD_IDEO_UNIFIED, self.0) }
    pub fn is_lowercase(self) -> bool { in_ranges(&UCD_CASE_IS_LOWER, self.0) }
    pub fn is_lowercase_other(self) -> bool { in_ranges(&UCD_CASE_IS_LOWER_OTHER, self.0) }
    pub fn is_pattern_syntax(self) -> bool { in_ranges(&UCD_PATT_SYNTAX, self.0) }
    pub fn is_pattern_whitespace(self) -> bool {
        match self.0 as u32 {
            9...13 | 32 | 133 | 8206 | 8207 | 8232 | 8233 => true,
            _ => false } }
    pub fn is_uppercase(self) -> bool { in_ranges(&UCD_CASE_IS_UPPER, self.0) }
    pub fn is_uppercase_other (self) -> bool {
        match self.0 as u32 {
            8544...8559 | 9398...9423 | 127280...127305 | 127312...127337 | 127344...127369 => true,
            _ => false } }
    pub fn quick_check_nfd(self) -> bool { !in_ranges(&UCD_QUICK_NFD, self.0) }
    pub fn quick_check_nfkd(self) -> bool { !in_ranges(&UCD_QUICK_NFKD, self.0) }

    //
    pub fn quick_check_nfc(self) -> Trilean { search_range(&UCD_QNFC, self.0).unwrap_or(Trilean::True) }
    pub fn quick_check_nfkc(self) -> Trilean {
        match in_ranges(&UCD_QNFKC, self.0) {
            true => Trilean::False,
            false => self.quick_check_nfc() } }

    pub fn uppercase_simple(self) -> char { search(&UCD_CASE_SIMP_UP, self.0).map(cp_decode).unwrap_or(self.codepoint()) }
    pub fn lowercase_simple(self) -> char { search(&UCD_CASE_SIMP_LW, self.0).map(cp_decode).unwrap_or(self.codepoint()) }
    pub fn titlecase_simple(self) -> char { search(&UCD_CASE_SIMP_TI, self.0).map(cp_decode).unwrap_or(self.codepoint()) }
    pub fn casefold_simple(self) -> char { search(&UCD_CASE_SIMP_FD, self.0).map(cp_decode).unwrap_or(self.codepoint()) }

    pub fn uppercase(self) -> CharIter { CharIter::new(search(&UCD_CASE_UP, self.0), self) }
    pub fn lowercase(self) -> CharIter { CharIter::new(search(&UCD_CASE_LW, self.0), self) }
    pub fn titlecase(self) -> CharIter { CharIter::new(search(&UCD_CASE_TI, self.0), self) }
    pub fn casefold(self) -> CharIter { CharIter::new(search(&UCD_CASE_FD, self.0), self) }
    pub fn casefold_nfkc(self) -> CharIter { CharIter::new(search(&UCD_CASE_FD_NFKC, self.0), self) }
    pub fn casefold_nfkc_closure(self) -> CharIter { CharIter::new(search(&UCD_CASE_FD_CLOS, self.0), self) }

    // decomp
    pub fn decomposition_map(self) -> CharIter { CharIter::new(search(&UCD_DECOMP_MAP, self.0), self) }
    pub fn decomposition_type(self) -> Option<DecompositionType> { search_range(&UCD_DECOMP_TYPE, self.0) }
    pub fn word_break(self) -> WordBreak { search_range(&UCD_WBRK, self.0).unwrap_or(WordBreak::Other) }
    pub fn sentence_break(self) -> SentenceBreak { search_range(&UCD_SBRK, self.0).unwrap_or(SentenceBreak::Other) }
    pub fn grapheme_cluster_break(self) -> GraphemeClusterBreak {
        let cx = self.0.clone();
        match self.hangul_syllable_type() {
            Some(HangulSyllableType::LeadingJamo) => GraphemeClusterBreak::LeadingJamo,
            Some(HangulSyllableType::VowelJamo) => GraphemeClusterBreak::VowelJamo,
            Some(HangulSyllableType::TrailingJamo) => GraphemeClusterBreak::TrailingJamo,
            Some(HangulSyllableType::LVSyllable) => GraphemeClusterBreak::LVHangulSyllable,
            Some(HangulSyllableType::LVTSyllable) => GraphemeClusterBreak::LVTHangulSyllable,
            None => search_range(&UCD_GCB, cx).unwrap_or(GraphemeClusterBreak::Other)
        }
    }
}
