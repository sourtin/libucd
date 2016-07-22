extern crate ucd;
use ucd::Codepoint;
use std::char;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn test<F>(path: &str, func: F)
    where F: Fn(usize, Codepoint, &str) -> ()
{
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut max: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        max = i;
        let line = line.unwrap();

        assert!(i < 0x110000);
        let c = unsafe { char::from_u32_unchecked(i as u32) };
        let cp = Codepoint(c);

        func(i, cp, &line);
    }

    assert!(max == 0x10ffff);
}

fn test_bool<F>(path: &str, func: F)
    where F: Fn(Codepoint) -> bool
{
    test(path, |i, cp, line| {
        let b1 = func(cp);
        let b2 = match line.chars().next() {
            Some('0') => false,
            Some('1') => true,
            _ => panic!("unexpected test data, line {}", i)
        };

        if b1 != b2 { panic!("{}", i); }
    });
}

fn test_oint<F>(path: &str, func: F)
    where F: Fn(Codepoint) -> Option<u32>
{
    test(path, |i, cp, line| {
        let c1 = func(cp);
        let c2 = line.parse::<u32>().ok();
        if c1 != c2 { panic!("{}: {:?} {:?}", i, c1, c2); }
    });
}

fn test_int<F>(path: &str, func: F)
    where F: Fn(Codepoint) -> u32
{ test_oint(path, |cp| Some(func(cp))); }

#[test] #[ignore]
fn age() { test("./tests/data/age.txt", |i, cp, line| {
    let chars = line.chars().collect::<Vec<char>>();

    let a1 = cp.age();
    let a2 = if chars[0] == '-' {
        None
    } else {
        Some((chars[0].to_digit(10).unwrap() as u8,
              chars[1].to_digit(10).unwrap() as u8))
    };

    if a1 != a2 { panic!("{}: {:?} {:?}", i, a1, a2); }
});}

#[test] #[ignore] fn block() { test_oint("./tests/data/block.txt", |cp| cp.block().map(|b| b as u32)); }
#[test] #[ignore] fn category() { test_int("./tests/data/cat.txt", |cp| cp.category() as u32); }
#[test] #[ignore] fn combining_class() { test_int("./tests/data/ccc.txt", |cp| cp.combining_class() as u32); }

#[test] #[ignore] fn bidi_control() { test_bool("./tests/data/bidi-control.txt", |cp| cp.bidi_is_control()); }
#[test] #[ignore] fn bidi_class() { test_int("./tests/data/bidi-class.txt", |cp| cp.bidi_class() as u32); }
#[test] #[ignore] fn bidi_mirrored() { test_bool("./tests/data/bidi-mirrored.txt", |cp| cp.bidi_is_mirrored()); }
#[test] #[ignore] fn bidi_mirror() { test_oint("./tests/data/bidi-mirror.txt", |cp| cp.bidi_mirror().map(|c| c as u32)); }
#[test] #[ignore] fn bidi_paired() { test_int("./tests/data/bidi-paired.txt", |cp| cp.bidi_paired_bracket() as u32); }
#[test] #[ignore]
fn bidi_paired_bracket_type() { test("./tests/data/bidi-bratype.txt", |i, cp, line| {
    let b1 = cp.bidi_paired_bracket_type();
    let b2 = match line.chars().next() {
        Some('-') => None,
        Some('(') => Some(ucd::BidiPairedBracketType::Open),
        Some(')') => Some(ucd::BidiPairedBracketType::Close),
        _ => panic!("unexpected test data, line {}", i)
    };

    if b1 != b2 { panic!("{}: {:?} {:?}", i, b1, b2); }
});}

#[test] #[ignore] fn ea_width() { test_int("./tests/data/eawidth.txt", |cp| cp.east_asian_width() as u32); }
#[test] #[ignore] fn linebreak() { test_oint("./tests/data/linebreak.txt", |cp| cp.linebreak_class().map(|c| c as u32)); }
#[test] #[ignore] fn deprecated() { test_bool("./tests/data/deprec.txt", |cp| cp.is_deprecated()); }
#[test] #[ignore] fn variation_selector() { test_bool("./tests/data/varsel.txt", |cp| cp.is_variation_selector()); }
#[test] #[ignore] fn noncharacter() { test_bool("./tests/data/nonchar.txt", |cp| cp.is_noncharacter()); }
#[test] #[ignore] fn numeric_type() { test_oint("./tests/data/numtype.txt", |cp| cp.numeric_type().map(|t| t as u32)); }
#[test] #[ignore]
fn numeric_value() { test("./tests/data/numval.txt", |i, cp, line| {
    let n1 = cp.numeric_value();

    let ls: Vec<&str> = line.splitn(2, ',').collect();
    let n2 = match ls[1].parse::<u32>().unwrap() {
        0 => None,
        1 => Some(ucd::Number::Integer(ls[0].parse::<i64>().unwrap())),
        den => Some(ucd::Number::Rational(ls[0].parse::<i32>().unwrap(), den))
    };

    if n1 != n2 { panic!("{}: {:?} {:?}", i, n1, n2); }
});}

#[test] #[ignore] fn join_control() { test_bool("./tests/data/joinctl.txt", |cp| cp.join_control()); }
#[test] #[ignore] fn joining_type() { test_int("./tests/data/jointyp.txt", |cp| cp.joining_type() as u32); }
#[test] #[ignore] fn joining_group() { test_int("./tests/data/joingrp.txt", |cp| cp.joining_group() as u32); }
#[test] #[ignore] fn hangul_syllable_type() { test_oint("./tests/data/hangulst.txt", |cp| cp.hangul_syllable_type().map(|c| c as u32)); }
#[test] #[ignore] fn indic_syllabic_category() { test_int("./tests/data/indic-sylcat.txt", |cp| cp.indic_syllabic_category() as u32); }
#[test] #[ignore] fn indic_positional_category() { test_oint("./tests/data/indic-poscat.txt", |cp| cp.indic_positional_category().map(|c| c as u32)); }
#[test] #[ignore] fn jamo_short_name() { test("./tests/data/jsn.txt", |i, cp, s2| {
    let s1 = cp.jamo_short_name().unwrap_or("-");
    if s1 != s1 { panic!("{}: {:?} {:?}", i, s1, s2); }
});}
#[test] #[ignore] fn script() { test_oint("./tests/data/script.txt", |cp| cp.script().map(|c| c as u32)); }
#[test] #[ignore]
fn script_extensions() { test("./tests/data/scrext.txt", |i, cp, line| {
    let x1 = cp.script_extensions().map(|s| s.iter().map(|sc| *sc as u32).collect::<Vec<u32>>());
    let x2: Option<Vec<u32>> = if line.len() > 0 {
        Some(line.split(' ').map(|v| v.parse::<u32>().unwrap()).collect())
    } else { None };
    if x1 != x2 { panic!("{}: {:?} {:?}", i, x1, x2); }
});}


#[test] #[ignore] fn is_ascii_hex_digit() { test_bool("./tests/data/hex-digit-ascii.txt", |cp| cp.is_ascii_hex_digit()); }
#[test] #[ignore] fn is_preprended_concatenation_mark() { test_bool("./tests/data/prepended-concatenation-mark.txt", |cp| cp.is_preprended_concatenation_mark()); }
#[test] #[ignore] fn is_hyphen() { test_bool("./tests/data/hyphen.txt", |cp| cp.is_hyphen()); }
#[test] #[ignore] fn is_hex_digit() { test_bool("./tests/data/hex-digit.txt", |cp| cp.is_hex_digit()); }
#[test] #[ignore] fn is_whitespace() { test_bool("./tests/data/white.txt", |cp| cp.is_whitespace()); }
#[test] #[ignore] fn is_logical_order_exception() { test_bool("./tests/data/logical-order-exception.txt", |cp| cp.is_logical_order_exception()); }
#[test] #[ignore] fn is_sentence_terminal() { test_bool("./tests/data/term-sentence.txt", |cp| cp.is_sentence_terminal()); }
#[test] #[ignore] fn is_dash() { test_bool("./tests/data/dash.txt", |cp| cp.is_dash()); }
#[test] #[ignore] fn is_quotation_mark() { test_bool("./tests/data/quot.txt", |cp| cp.is_quotation_mark()); }
#[test] #[ignore] fn is_terminal_punctutation() { test_bool("./tests/data/term-punc.txt", |cp| cp.is_terminal_punctutation()); }
#[test] #[ignore] fn is_extender() { test_bool("./tests/data/extender.txt", |cp| cp.is_extender()); }
#[test] #[ignore] fn is_soft_dotted() { test_bool("./tests/data/soft-dotted.txt", |cp| cp.is_soft_dotted()); }
#[test] #[ignore] fn is_default_ignorable() { test_bool("./tests/data/default-ignorable.txt", |cp| cp.is_default_ignorable()); }
#[test] #[ignore] fn is_alphabetic() { test_bool("./tests/data/alpha.txt", |cp| cp.is_alphabetic()); }
#[test] #[ignore] fn is_default_ignorable_other() { test_bool("./tests/data/default-ignorable-other.txt", |cp| cp.is_default_ignorable_other()); }
#[test] #[ignore] fn is_math_other() { test_bool("./tests/data/math-other.txt", |cp| cp.is_math_other()); }
#[test] #[ignore] fn is_diacritic() { test_bool("./tests/data/diacritic.txt", |cp| cp.is_diacritic()); }
#[test] #[ignore] fn is_math() { test_bool("./tests/data/math.txt", |cp| cp.is_math()); }
#[test] #[ignore] fn is_alphabetic_other() { test_bool("./tests/data/alpha-other.txt", |cp| cp.is_alphabetic_other()); }

#[test] #[ignore] fn changes_when_casefolded() { test_bool("./tests/data/case-changes-casefold.txt", |cp| cp.changes_when_casefolded()); }
#[test] #[ignore] fn changes_when_casefolded_nfkc() { test_bool("./tests/data/case-changes-casefold-nfkc.txt", |cp| cp.changes_when_casefolded_nfkc()); }
#[test] #[ignore] fn changes_when_casemapped() { test_bool("./tests/data/case-changes-casemap.txt", |cp| cp.changes_when_casemapped()); }
#[test] #[ignore] fn changes_when_lowercased() { test_bool("./tests/data/case-changes-lower.txt", |cp| cp.changes_when_lowercased()); }
#[test] #[ignore] fn changes_when_titlecased() { test_bool("./tests/data/case-changes-title.txt", |cp| cp.changes_when_titlecased()); }
#[test] #[ignore] fn changes_when_uppercased() { test_bool("./tests/data/case-changes-upper.txt", |cp| cp.changes_when_uppercased()); }
#[test] #[ignore] fn excluded_from_composition() { test_bool("./tests/data/comp-excl.txt", |cp| cp.excluded_from_composition()); }
#[test] #[ignore] fn excluded_from_composition_fully() { test_bool("./tests/data/comp-excl-full.txt", |cp| cp.excluded_from_composition_fully()); }
#[test] #[ignore] fn expands_on_nfc() { test_bool("./tests/data/expanding-nfc.txt", |cp| cp.expands_on_nfc()); }
#[test] #[ignore] fn expands_on_nfd() { test_bool("./tests/data/expanding-nfd.txt", |cp| cp.expands_on_nfd()); }
#[test] #[ignore] fn expands_on_nfkc() { test_bool("./tests/data/expanding-nfkc.txt", |cp| cp.expands_on_nfkc()); }
#[test] #[ignore] fn expands_on_nfkd() { test_bool("./tests/data/expanding-nfkd.txt", |cp| cp.expands_on_nfkd()); }
#[test] #[ignore] fn is_case_ignorable() { test_bool("./tests/data/case-ignorable.txt", |cp| cp.is_case_ignorable()); }
#[test] #[ignore] fn is_cased() { test_bool("./tests/data/cased.txt", |cp| cp.is_cased()); }
#[test] #[ignore] fn is_grapheme_base() { test_bool("./tests/data/graph-base.txt", |cp| cp.is_grapheme_base()); }
#[test] #[ignore] fn is_grapheme_extend() { test_bool("./tests/data/graph-ext.txt", |cp| cp.is_grapheme_extend()); }
#[test] #[ignore] fn is_grapheme_extend_other() { test_bool("./tests/data/graph-ext-other.txt", |cp| cp.is_grapheme_extend_other()); }
#[test] #[ignore] fn is_grapheme_link() { test_bool("./tests/data/graph-link.txt", |cp| cp.is_grapheme_link()); }
#[test] #[ignore] fn is_id_continue() { test_bool("./tests/data/id-cont.txt", |cp| cp.is_id_continue()); }
#[test] #[ignore] fn is_id_continue_nfkc() { test_bool("./tests/data/id-cont-nfkc.txt", |cp| cp.is_id_continue_nfkc()); }
#[test] #[ignore] fn is_id_continue_other() { test_bool("./tests/data/id-cont-other.txt", |cp| cp.is_id_continue_other()); }
#[test] #[ignore] fn is_id_start() { test_bool("./tests/data/id-start.txt", |cp| cp.is_id_start()); }
#[test] #[ignore] fn is_id_start_nfkc() { test_bool("./tests/data/id-start-nfkc.txt", |cp| cp.is_id_start_nfkc()); }
#[test] #[ignore] fn is_id_start_other() { test_bool("./tests/data/id-start-other.txt", |cp| cp.is_id_start_other()); }
#[test] #[ignore] fn is_ideograph() { test_bool("./tests/data/ideo.txt", |cp| cp.is_ideograph()); }
#[test] #[ignore] fn is_ideograph_description_sequence_binary_operator() { test_bool("./tests/data/ideo-desc-seq-bin-op.txt", |cp| cp.is_ideograph_description_sequence_binary_operator()); }
#[test] #[ignore] fn is_ideograph_description_sequence_radical() { test_bool("./tests/data/ideo-desc-seq-radical.txt", |cp| cp.is_ideograph_description_sequence_radical()); }
#[test] #[ignore] fn is_ideograph_description_sequence_trinary_operator() { test_bool("./tests/data/ideo-desc-seq-trin-op.txt", |cp| cp.is_ideograph_description_sequence_trinary_operator()); }
#[test] #[ignore] fn is_ideograph_unified() { test_bool("./tests/data/ideo-unified.txt", |cp| cp.is_ideograph_unified()); }
#[test] #[ignore] fn is_lowercase() { test_bool("./tests/data/case-is-lower.txt", |cp| cp.is_lowercase()); }
#[test] #[ignore] fn is_lowercase_other() { test_bool("./tests/data/case-is-lower-other.txt", |cp| cp.is_lowercase_other()); }
#[test] #[ignore] fn is_pattern_syntax() { test_bool("./tests/data/patt-syntax.txt", |cp| cp.is_pattern_syntax()); }
#[test] #[ignore] fn is_pattern_whitespace() { test_bool("./tests/data/patt-white.txt", |cp| cp.is_pattern_whitespace()); }
#[test] #[ignore] fn is_uppercase() { test_bool("./tests/data/case-is-upper.txt", |cp| cp.is_uppercase()); }
#[test] #[ignore] fn is_uppercase_other() { test_bool("./tests/data/case-is-upper-other.txt", |cp| cp.is_uppercase_other()); }
#[test] #[ignore] fn quick_check_nfd() { test_bool("./tests/data/quick-nfd.txt", |cp| cp.quick_check_nfd()); }
#[test] #[ignore] fn quick_check_nfkd() { test_bool("./tests/data/quick-nfkd.txt", |cp| cp.quick_check_nfkd()); }

#[test] fn quick_check_nfc() { test_int("./tests/data/qnfc.txt", |cp| cp.quick_check_nfc() as u32); }
#[test] fn quick_check_nfkc() { test_int("./tests/data/qnfkc.txt", |cp| cp.quick_check_nfkc() as u32); }
