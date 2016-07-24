#![no_std]
use core::cmp::Ordering::{Equal, Less, Greater};
use core::char;
use core::slice;

pub mod tables;
pub use tables::{
    BidiClass,
    BidiPairedBracketType,
    DecompositionType,
    EastAsianWidth,
    GraphemeClusterBreak,
    HangulSyllableType,
    IndicPositionalCategory,
    IndicSyllabicCategory,
    JoiningGroup,
    JoiningType,
    LinebreakClass,
    NumericType,
    Script,
    SentenceBreak,
    Trilean,
    UnicodeBlock,
    UnicodeCategory,
    WordBreak
};

// for use with numeric_value
#[derive(Clone,Copy,Eq,PartialEq,Debug,Ord,PartialOrd)]
pub enum Number {
    Integer(i64),
    Rational(i32,u32)
}

fn search_range<S: Clone>(table: &[((u8,u8,u8),(u8,u8,u8),S)], cp: char) -> Option<S> {
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

fn search<S: Clone>(table: &[((u8,u8,u8),S)], cp: char) -> Option<S> {
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
    match search_range(table, cp) {
        Some(()) => true,
        None => false
    }
}

fn in_table(table: &[((u8,u8,u8),())], cp: char) -> bool {
    match search(table, cp) {
        Some(()) => true,
        None => false
    }
}

fn map16(table: &[(u16,u16)], cp: char) -> Option<char> {
    let ca = cp as u32;
    let cb = ca as u16;

    if ca > 65536 { return None; }

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
    pub fn new(osl: Option<&'static [(u8,u8,u8)]>, cp: char) -> CharIter {
        CharIter(match osl {
            Some(sl) => CharIterInternal::Iterator(sl.iter()),
            None => CharIterInternal::Single(cp)
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

pub trait Codepoint {
    // general
    fn age(self) -> Option<(u8,u8)>;
    fn block(self) -> Option<UnicodeBlock>;
    fn category(self) -> UnicodeCategory;
    fn codepoint(self) -> char;
    fn iso_comment(self) -> &'static str;

    // function and appearance
    fn is_alphabetic(self) -> bool;
    fn is_alphabetic_other(self) -> bool;
    fn is_dash(self) -> bool;
    fn is_default_ignorable(self) -> bool;
    fn is_default_ignorable_other(self) -> bool;
    fn is_deprecated(self) -> bool;
    fn is_diacritic(self) -> bool;
    fn is_extender(self) -> bool;
    fn is_hex_digit(self) -> bool;
    fn is_hex_digit_ascii(self) -> bool;
    fn is_hyphen(self) -> bool;
    fn is_logical_order_exception(self) -> bool;
    fn is_math(self) -> bool;
    fn is_math_other(self) -> bool;
    fn is_noncharacter(self) -> bool;
    fn is_preprended_concatenation_mark(self) -> bool;
    fn is_quotation_mark(self) -> bool;
    fn is_sentence_terminal(self) -> bool;
    fn is_soft_dotted(self) -> bool;
    fn is_terminal_punctuation(self) -> bool;
    fn is_variation_selector(self) -> bool;
    fn is_whitespace(self) -> bool;

    // numeric
    fn numeric_type(self) -> Option<NumericType>;
    fn numeric_value(self) -> Option<Number>;

    // identifiers and syntax
    fn is_id_continue(self) -> bool;
    fn is_id_continue_nfkc(self) -> bool;
    fn is_id_continue_other(self) -> bool;
    fn is_id_start(self) -> bool;
    fn is_id_start_nfkc(self) -> bool;
    fn is_id_start_other(self) -> bool;
    fn is_pattern_syntax(self) -> bool;
    fn is_pattern_whitespace(self) -> bool;

    // scripts
    fn east_asian_width(self) -> EastAsianWidth;
    fn hangul_syllable_type(self) -> Option<HangulSyllableType>;
    fn jamo_short_name(self) -> Option<&'static str>;
    fn indic_positional_category(self) -> Option<IndicPositionalCategory>;
    fn indic_syllabic_category(self) -> IndicSyllabicCategory;
    fn is_ideograph(self) -> bool;
    fn is_ideograph_description_sequence_binary_operator(self) -> bool;
    fn is_ideograph_description_sequence_radical(self) -> bool;
    fn is_ideograph_description_sequence_trinary_operator(self) -> bool;
    fn is_ideograph_unified(self) -> bool;
    fn join_control(self) -> bool;
    fn joining_group(self) -> JoiningGroup;
    fn joining_type(self) -> JoiningType;
    fn script(self) -> Option<Script>;
    fn script_extensions(self) -> Option<&'static [Script]>;

    // bidirectionality
    fn bidi_class(self) -> BidiClass;
    fn bidi_is_control(self) -> bool;
    fn bidi_is_mirrored(self) -> bool;
    fn bidi_mirror(self) -> Option<char>;
    fn bidi_paired_bracket(self) -> char;
    fn bidi_paired_bracket_type(self) -> Option<BidiPairedBracketType>;

    // case
    fn casefold(self) -> CharIter;
    fn casefold_nfkc(self) -> CharIter;
    fn casefold_nfkc_closure(self) -> CharIter;
    fn casefold_simple(self) -> char;
    fn changes_when_casefolded(self) -> bool;
    fn changes_when_casefolded_nfkc(self) -> bool;
    fn changes_when_casemapped(self) -> bool;
    fn changes_when_lowercased(self) -> bool;
    fn changes_when_titlecased(self) -> bool;
    fn changes_when_uppercased(self) -> bool;
    fn is_case_ignorable(self) -> bool;
    fn is_cased(self) -> bool;
    fn is_lowercase(self) -> bool;
    fn is_lowercase_other(self) -> bool;
    fn is_uppercase(self) -> bool;
    fn is_uppercase_other (self) -> bool;
    fn lowercase(self) -> CharIter;
    fn lowercase_simple(self) -> char;
    fn titlecase(self) -> CharIter;
    fn titlecase_simple(self) -> char;
    fn uppercase(self) -> CharIter;
    fn uppercase_simple(self) -> char;

    // normalisation
    fn canonical_combining_class(self) -> u8;
    fn decomposition_map(self) -> CharIter;
    fn decomposition_type(self) -> Option<DecompositionType>;
    fn excluded_from_composition(self) -> bool;
    fn excluded_from_composition_fully(self) -> bool;
    fn expands_on_nfc(self) -> bool;
    fn expands_on_nfd(self) -> bool;
    fn expands_on_nfkc(self) -> bool;
    fn expands_on_nfkd(self) -> bool;
    fn quick_check_nfc(self) -> Trilean;
    fn quick_check_nfd(self) -> bool;
    fn quick_check_nfkc(self) -> Trilean;
    fn quick_check_nfkd(self) -> bool;

    // segmentation
    fn grapheme_cluster_break(self) -> GraphemeClusterBreak;
    fn is_grapheme_base(self) -> bool;
    fn is_grapheme_extend(self) -> bool;
    fn is_grapheme_extend_other(self) -> bool;
    fn is_grapheme_link(self) -> bool;
    fn linebreak_class(self) -> Option<LinebreakClass>;
    fn sentence_break(self) -> SentenceBreak;
    fn word_break(self) -> WordBreak;
}

impl Codepoint for char {
    // general
    fn age(self) -> Option<(u8,u8)> {
        search_range(&tables::UCD_AGE, self)
    }

    fn block(self) -> Option<UnicodeBlock> {
        search_range(&tables::UCD_BLOCK, self)
    }

    fn category(self) -> UnicodeCategory {
        search_range(&tables::UCD_CAT, self)
            .unwrap_or(UnicodeCategory::Unassigned)
    }

    fn codepoint(self) -> char {
        self
    }

    fn iso_comment(self) -> &'static str {
        ""
    }




    // function and appearance
    fn is_alphabetic(self) -> bool {
        in_ranges(&tables::UCD_ALPHA, self)
    }

    fn is_alphabetic_other(self) -> bool {
        in_ranges(&tables::UCD_ALPHA_OTHER, self)
    }

    fn is_dash(self) -> bool {
        in_table(&tables::UCD_DASH, self)
    }

    fn is_default_ignorable(self) -> bool {
        in_ranges(&tables::UCD_DEFAULT_IGNORABLE, self)
    }

    fn is_default_ignorable_other(self) -> bool {
        in_ranges(&tables::UCD_DEFAULT_IGNORABLE_OTHER, self)
    }

    fn is_deprecated(self) -> bool {
        match self as u32 {
            329 | 1651 | 3959 | 3961 | 6051 | 6052 |
                  8298...8303 | 9001 | 9002 | 917505 => true,
            _ => false
        }
    }
    fn is_diacritic(self) -> bool {
        in_ranges(&tables::UCD_DIACRITIC, self)
    }

    fn is_extender(self) -> bool {
        in_table(&tables::UCD_EXTENDER, self)
    }

    fn is_hex_digit(self) -> bool {
        in_table(&tables::UCD_HEX_DIGIT, self)
    }

    fn is_hex_digit_ascii(self) -> bool {
        in_table(&tables::UCD_HEX_DIGIT_ASCII, self)
    }

    fn is_hyphen(self) -> bool {
        in_table(&tables::UCD_HYPHEN, self)
    }

    fn is_logical_order_exception(self) -> bool {
        in_table(&tables::UCD_LOGICAL_ORDER_EXCEPTION, self)
    }

    fn is_math(self) -> bool {
        in_ranges(&tables::UCD_MATH, self)
    }

    fn is_math_other(self) -> bool {
        in_ranges(&tables::UCD_MATH_OTHER, self)
    }

    fn is_noncharacter(self) -> bool {
        let cp = self as u32;
        (cp >= 0xfdd0 && cp <= 0xfdef)
            || ((cp & 0xffff) >= 0xfffe)
    }

    fn is_preprended_concatenation_mark(self) -> bool {
        in_table(&tables::UCD_PREPENDED_CONCATENATION_MARK, self)
    }

    fn is_quotation_mark(self) -> bool {
        in_table(&tables::UCD_QUOT, self)
    }

    fn is_sentence_terminal(self) -> bool {
        in_table(&tables::UCD_TERM_SENTENCE, self)
    }

    fn is_soft_dotted(self) -> bool {
        in_table(&tables::UCD_SOFT_DOTTED, self)
    }

    fn is_terminal_punctuation(self) -> bool {
        in_table(&tables::UCD_TERM_PUNC, self)
    }

    fn is_variation_selector(self) -> bool {
        let cp = self as u32;
        (cp >= 917760 && cp <= 917999)
            || (cp >= 65024 && cp <= 65039)
            || (cp >= 6155 && cp <= 6157)
    }

    fn is_whitespace(self) -> bool {
        in_table(&tables::UCD_WHITE, self)
    }




    // numeric
    fn numeric_type(self) -> Option<NumericType> {
        search_range(&tables::UCD_NUMTYPE, self)
    }

    fn numeric_value(self) -> Option<Number> {
        search(&tables::UCD_NUMVAL, self).map(|i| {
            match tables::UCD_NUMS[i as usize] {
                (num, 1) => Number::Integer(num),
                (num, den) => Number::Rational(num as i32, den as u32)
            }
        })
    }



    // identifiers and syntax
    fn is_id_continue(self) -> bool {
        in_ranges(&tables::UCD_ID_CONT, self)
    }

    fn is_id_continue_nfkc(self) -> bool {
        in_ranges(&tables::UCD_ID_CONT_NFKC, self)
    }

    fn is_id_continue_other(self) -> bool {
         match self as u32 {
            183 | 903 | 4969...4977 | 6618 => true,
            _ => false
        }
    }

    fn is_id_start(self) -> bool {
        in_ranges(&tables::UCD_ID_START, self)
    }

    fn is_id_start_nfkc(self) -> bool {
        in_ranges(&tables::UCD_ID_START_NFKC, self)
    }

    fn is_id_start_other(self) -> bool {
        match self as u32 {
            6277 | 6278 | 8472 | 8494| 12443 | 12444 => true,
            _ => false
        }
    }

    fn is_pattern_syntax(self) -> bool {
        in_ranges(&tables::UCD_PATT_SYNTAX, self)
    }

    fn is_pattern_whitespace(self) -> bool {
        match self as u32 {
            9...13 | 32 | 133 | 8206
                   | 8207 | 8232 | 8233 => true,
            _ => false
        }
    }




    // scripts
    fn east_asian_width(self) -> EastAsianWidth {
        search_range(&tables::UCD_EAWIDTH, self)
            .unwrap_or(EastAsianWidth::Neutral)
    }

    fn hangul_syllable_type(self) -> Option<HangulSyllableType> {
        let cp = self as u32;
        match cp {
            4352...4447 | 43360...43388 => Some(HangulSyllableType::LeadingJamo),
            4448...4519 | 55216...55238 => Some(HangulSyllableType::VowelJamo),
            4520...4607 | 55243...55291 => Some(HangulSyllableType::TrailingJamo),
            44032...55203 => Some({
                if cp % 28 == 16 { HangulSyllableType::LVSyllable }
                else { HangulSyllableType::LVTSyllable }
            }),
            _ => None
        }
    }

    fn jamo_short_name(self) -> Option<&'static str> {
        search(&tables::UCD_JSN, self)
    }

    fn indic_positional_category(self) -> Option<IndicPositionalCategory> {
        search(&tables::UCD_INPC, self)
    }

    fn indic_syllabic_category(self) -> IndicSyllabicCategory {
        search_range(&tables::UCD_INSC, self)
            .unwrap_or(IndicSyllabicCategory::Other)
    }

    fn is_ideograph(self) -> bool {
        in_ranges(&tables::UCD_IDEO, self)
    }

    fn is_ideograph_description_sequence_binary_operator(self) -> bool {
        match self as u32 {
            12272 | 12273 | 12276...12283 => true,
            _ => false
        }
    }

    fn is_ideograph_description_sequence_radical(self) -> bool {
        match self as u32 {
            11904...11929 | 11931...12019 | 12032...12245 => true,
            _ => false
        }
    }

    fn is_ideograph_description_sequence_trinary_operator(self) -> bool {
        let cp = self as u32;
        cp == 12274 || cp == 12275
    }

    fn is_ideograph_unified(self) -> bool {
        in_ranges(&tables::UCD_IDEO_UNIFIED, self)
    }

    fn join_control(self) -> bool {
        let cp = self as u32;
        cp == 8204 || cp == 8205
    }

    fn joining_group(self) -> JoiningGroup {
        search(&tables::UCD_JOINGRP, self)
            .unwrap_or(JoiningGroup::NoJoiningGroup)
    }

    fn joining_type(self) -> JoiningType {
        search_range(&tables::UCD_JOINTYPE, self)
            .unwrap_or(JoiningType::NonJoining)
    }

    fn script(self) -> Option<Script> {
        search_range(&tables::UCD_SCRIPT, self)
    }

    fn script_extensions(self) -> Option<&'static [Script]> {
        match search(&tables::UCD_SCRIPTEXT, self) {
            None => self.script().map(
                |s| tables::UCD_SCRIPT_MAP[s as usize]),
            x => x
        }
    }




    // bidirectionality
    fn bidi_class(self) -> BidiClass {
        search_range(&tables::UCD_BIDI_CLASS, self)
            .unwrap_or(BidiClass::LeftToRight)
    }

    fn bidi_is_control(self) -> bool {
        match self as u32 {
            1564 | 8206 | 8207 | 8234...8238 | 8294...8297 => true,
            _ => false
        }
    }

    fn bidi_is_mirrored(self) -> bool {
        in_ranges(&tables::UCD_BIDI_MIRRORED, self)
    }

    fn bidi_mirror(self) -> Option<char> {
        map16(&tables::UCD_BIDI_MIRROR, self)
    }

    fn bidi_paired_bracket(self) -> char {
        map16(&tables::UCD_BIDI_PAIRED, self)
            .unwrap_or(self)
    }

    fn bidi_paired_bracket_type(self) -> Option<BidiPairedBracketType> {
        search(&tables::UCD_BIDI_BRATYPE, self)
    }




    // case
    fn casefold(self) -> CharIter {
        CharIter::new(search(&tables::UCD_CASE_FD, self), self)
    }

    fn casefold_nfkc(self) -> CharIter {
        CharIter::new(search(&tables::UCD_CASE_FD_NFKC, self), self)
    }

    fn casefold_nfkc_closure(self) -> CharIter {
        CharIter::new(search(&tables::UCD_CASE_FD_CLOS, self), self)
    }

    fn casefold_simple(self) -> char {
        search(&tables::UCD_CASE_SIMP_FD, self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }

    fn changes_when_casefolded(self) -> bool {
        in_table(&tables::UCD_CASE_CHANGES_CASEFOLD, self)
    }

    fn changes_when_casefolded_nfkc(self) -> bool {
        in_ranges(&tables::UCD_CASE_CHANGES_CASEFOLD_NFKC, self)
    }

    fn changes_when_casemapped(self) -> bool {
        in_ranges(&tables::UCD_CASE_CHANGES_CASEMAP, self)
    }

    fn changes_when_lowercased(self) -> bool {
        in_table(&tables::UCD_CASE_CHANGES_LOWER, self)
    }

    fn changes_when_titlecased(self) -> bool {
        in_table(&tables::UCD_CASE_CHANGES_TITLE, self)
    }

    fn changes_when_uppercased(self) -> bool {
        in_table(&tables::UCD_CASE_CHANGES_UPPER, self)
    }

    fn is_case_ignorable(self) -> bool {
        in_ranges(&tables::UCD_CASE_IGNORABLE, self)
    }

    fn is_cased(self) -> bool {
        in_ranges(&tables::UCD_CASED, self)
    }

    fn is_lowercase(self) -> bool {
        in_ranges(&tables::UCD_CASE_IS_LOWER, self)
    }

    fn is_lowercase_other(self) -> bool {
        in_ranges(&tables::UCD_CASE_IS_LOWER_OTHER, self)
    }

    fn is_uppercase(self) -> bool {
        in_ranges(&tables::UCD_CASE_IS_UPPER, self)
    }

    fn is_uppercase_other (self) -> bool {
        match self as u32 {
            8544...8559 | 9398...9423 | 127280...127305
                        | 127312...127337 | 127344...127369 => true,
            _ => false
        }
    }

    fn lowercase(self) -> CharIter {
        CharIter::new(search(&tables::UCD_CASE_LW, self), self)
    }

    fn lowercase_simple(self) -> char {
        search(&tables::UCD_CASE_SIMP_LW, self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }

    fn titlecase(self) -> CharIter {
        CharIter::new(search(&tables::UCD_CASE_TI, self), self)
    }

    fn titlecase_simple(self) -> char {
        search(&tables::UCD_CASE_SIMP_TI, self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }

    fn uppercase(self) -> CharIter {
        CharIter::new(search(&tables::UCD_CASE_UP, self), self)
    }

    fn uppercase_simple(self) -> char {
        search(&tables::UCD_CASE_SIMP_UP, self)
            .map(cp_decode)
            .unwrap_or(self.codepoint())
    }




    // normalisation
    fn canonical_combining_class(self) -> u8 {
        search_range(&tables::UCD_COMBCLS, self)
            .unwrap_or(0)
    }

    fn decomposition_map(self) -> CharIter {
        CharIter::new(search(&tables::UCD_DECOMP_MAP, self), self)
    }

    fn decomposition_type(self) -> Option<DecompositionType> {
        search_range(&tables::UCD_DECOMP_TYPE, self)
    }

    fn excluded_from_composition(self) -> bool {
        in_table(&tables::UCD_COMP_EXCL, self)
    }

    fn excluded_from_composition_fully(self) -> bool {
        in_ranges(&tables::UCD_COMP_EXCL_FULL, self)
    }

    fn expands_on_nfc(self) -> bool {
        in_table(&tables::UCD_EXPANDING_NFC, self)
    }

    fn expands_on_nfd(self) -> bool {
        in_ranges(&tables::UCD_EXPANDING_NFD, self)
    }

    fn expands_on_nfkc(self) -> bool {
        in_ranges(&tables::UCD_EXPANDING_NFKC, self)
    }

    fn expands_on_nfkd(self) -> bool {
        in_ranges(&tables::UCD_EXPANDING_NFKD, self)
    }

    fn quick_check_nfc(self) -> Trilean {
        search_range(&tables::UCD_QNFC, self)
            .unwrap_or(Trilean::True)
    }

    fn quick_check_nfd(self) -> bool {
        !in_ranges(&tables::UCD_QUICK_NFD, self)
    }

    fn quick_check_nfkc(self) -> Trilean {
        match in_ranges(&tables::UCD_QNFKC, self) {
            true => Trilean::False,
            false => self.quick_check_nfc()
        }
    }

    fn quick_check_nfkd(self) -> bool {
        !in_ranges(&tables::UCD_QUICK_NFKD, self)
    }




    // segmentation
    fn grapheme_cluster_break(self) -> GraphemeClusterBreak {
        let cx = self.clone();
        match self.hangul_syllable_type() {
            Some(HangulSyllableType::LeadingJamo)  => GraphemeClusterBreak::LeadingJamo,
            Some(HangulSyllableType::VowelJamo)    => GraphemeClusterBreak::VowelJamo,
            Some(HangulSyllableType::TrailingJamo) => GraphemeClusterBreak::TrailingJamo,
            Some(HangulSyllableType::LVSyllable)   => GraphemeClusterBreak::LVHangulSyllable,
            Some(HangulSyllableType::LVTSyllable)  => GraphemeClusterBreak::LVTHangulSyllable,
            None => search_range(&tables::UCD_GCB, cx)
                        .unwrap_or(GraphemeClusterBreak::Other)
        }
    }

    fn is_grapheme_base(self) -> bool {
        in_ranges(&tables::UCD_GRAPH_BASE, self)
    }

    fn is_grapheme_extend(self) -> bool {
        in_ranges(&tables::UCD_GRAPH_EXT, self)
    }

    fn is_grapheme_extend_other(self) -> bool {
        in_ranges(&tables::UCD_GRAPH_EXT_OTHER, self)
    }

    fn is_grapheme_link(self) -> bool {
        in_table(&tables::UCD_GRAPH_LINK, self)
    }

    fn linebreak_class(self) -> Option<LinebreakClass> {
        search_range(&tables::UCD_LB, self)
    }

    fn sentence_break(self) -> SentenceBreak {
        search_range(&tables::UCD_SBRK, self)
            .unwrap_or(SentenceBreak::Other)
    }

    fn word_break(self) -> WordBreak {
        search_range(&tables::UCD_WBRK, self)
            .unwrap_or(WordBreak::Other)
    }
}