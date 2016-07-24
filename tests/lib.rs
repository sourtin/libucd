extern crate ucd;
use ucd::Codepoint;
use std::char;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn test<F>(path: &str, func: F)
    where F: Fn(usize, char, &str) -> ()
{
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut max: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        max = i;
        let line = line.unwrap();

        assert!(i < 0x110000);
        let cp = unsafe { char::from_u32_unchecked(i as u32) };

        func(i, cp, &line);
    }

    // make sure the test file covers the full range
    assert!(max == 0x10ffff);
}

fn test_bool<F>(path: &str, func: F)
    where F: Fn(char) -> bool
{
    test(path, |i, cp, line| {
        let b1 = func(cp);
        let b2 = match line.chars().next() {
            Some('0') => false,
            Some('1') => true,
            _ => panic!("unexpected test data, line {}", i)
        };

        if b1 != b2 { panic!("{}: {:?} {:?}", i, b1, b2); }
    });
}

fn test_oint<F>(path: &str, func: F)
    where F: Fn(char) -> Option<u32>
{
    test(path, |i, cp, line| {
        let c1 = func(cp);
        let c2 = line.parse::<u32>().ok();
        if c1 != c2 { panic!("{}: {:?} {:?}", i, c1, c2); }
    });
}

fn test_int<F>(path: &str, func: F)
    where F: Fn(char) -> u32
{
    test_oint(path, |cp| Some(func(cp)));
}

fn test_cps<F>(path: &str, func: F)
    where F: Fn(char) -> ucd::CharIter
{
    test(path, |i, cp, line| {
        let x1: Vec<u32> = func(cp).map(|c| c as u32).collect();
        let x2: Vec<u32> = if line.len() > 0 {
            line.split(' ').map(|v| v.parse::<u32>().unwrap()).collect()
        } else {
            Vec::new()
        };
    
        if x1 != x2 { panic!("{}: {:?} {:?}", i, x1, x2); }
    });
}

// ... begin tests ... //

// general

#[test]
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

#[test]
fn block() {
    test_oint("./tests/data/block.txt",
        |cp| cp.block().map(|b| b as u32));
}

#[test]
fn category() {
    test_int("./tests/data/cat.txt",
        |cp| cp.category() as u32);
}



// function and appearance

#[test]
fn is_alphabetic() {
    test_bool("./tests/data/alpha.txt",
        |cp| Codepoint::is_alphabetic(cp));
}

#[test]
fn is_alphabetic_other() {
    test_bool("./tests/data/alpha-other.txt",
        |cp| cp.is_alphabetic_other());
}

#[test]
fn is_dash() {
    test_bool("./tests/data/dash.txt",
        |cp| cp.is_dash());
}

#[test]
fn is_default_ignorable() {
    test_bool("./tests/data/default-ignorable.txt",
        |cp| cp.is_default_ignorable());
}

#[test]
fn is_default_ignorable_other() {
    test_bool("./tests/data/default-ignorable-other.txt",
        |cp| cp.is_default_ignorable_other());
}

#[test]
fn is_deprecated() {
    test_bool("./tests/data/deprec.txt",
        |cp| cp.is_deprecated());
}

#[test]
fn is_diacritic() {
    test_bool("./tests/data/diacritic.txt",
        |cp| cp.is_diacritic());
}

#[test]
fn is_extender() {
    test_bool("./tests/data/extender.txt",
        |cp| cp.is_extender());
}

#[test]
fn is_hex_digit() {
    test_bool("./tests/data/hex-digit.txt",
        |cp| cp.is_hex_digit());
}

#[test]
fn is_hex_digit_ascii() {
    test_bool("./tests/data/hex-digit-ascii.txt",
        |cp| cp.is_hex_digit_ascii());
}

#[test]
fn is_hyphen() {
    test_bool("./tests/data/hyphen.txt",
        |cp| cp.is_hyphen());
}

#[test]
fn is_logical_order_exception() {
    test_bool("./tests/data/logical-order-exception.txt",
        |cp| cp.is_logical_order_exception());
}

#[test]
fn is_math() {
    test_bool("./tests/data/math.txt",
        |cp| cp.is_math());
}

#[test]
fn is_math_other() {
    test_bool("./tests/data/math-other.txt",
        |cp| cp.is_math_other());
}

#[test]
fn is_noncharacter() {
    test_bool("./tests/data/nonchar.txt",
        |cp| cp.is_noncharacter());
}

#[test]
fn is_preprended_concatenation_mark() {
    test_bool("./tests/data/prepended-concatenation-mark.txt",
        |cp| cp.is_preprended_concatenation_mark());
}

#[test]
fn is_quotation_mark() {
    test_bool("./tests/data/quot.txt",
        |cp| cp.is_quotation_mark());
}

#[test]
fn is_sentence_terminal() {
    test_bool("./tests/data/term-sentence.txt",
        |cp| cp.is_sentence_terminal());
}

#[test]
fn is_soft_dotted() {
    test_bool("./tests/data/soft-dotted.txt",
        |cp| cp.is_soft_dotted());
}

#[test]
fn is_terminal_punctuation() {
    test_bool("./tests/data/term-punc.txt",
        |cp| cp.is_terminal_punctuation());
}

#[test]
fn is_whitespace() {
    test_bool("./tests/data/white.txt",
        |cp| Codepoint::is_whitespace(cp));
}

#[test]
fn is_variation_selector() {
    test_bool("./tests/data/varsel.txt",
        |cp| cp.is_variation_selector());
}



// numeric

#[test]
fn numeric_type() {
    test_oint("./tests/data/numtype.txt",
        |cp| cp.numeric_type().map(|t| t as u32));
}

#[test]
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



// identifiers and syntax

#[test]
fn is_id_continue() {
    test_bool("./tests/data/id-cont.txt",
        |cp| cp.is_id_continue());
}

#[test]
fn is_id_continue_nfkc() {
    test_bool("./tests/data/id-cont-nfkc.txt",
        |cp| cp.is_id_continue_nfkc());
}

#[test]
fn is_id_continue_other() {
    test_bool("./tests/data/id-cont-other.txt",
        |cp| cp.is_id_continue_other());
}

#[test]
fn is_id_start() {
    test_bool("./tests/data/id-start.txt",
        |cp| cp.is_id_start());
}

#[test]
fn is_id_start_nfkc() {
    test_bool("./tests/data/id-start-nfkc.txt",
        |cp| cp.is_id_start_nfkc());
}

#[test]
fn is_id_start_other() {
    test_bool("./tests/data/id-start-other.txt",
        |cp| cp.is_id_start_other());
}

#[test]
fn is_pattern_syntax() {
    test_bool("./tests/data/patt-syntax.txt",
        |cp| cp.is_pattern_syntax());
}

#[test]
fn is_pattern_whitespace() {
    test_bool("./tests/data/patt-white.txt",
        |cp| cp.is_pattern_whitespace());
}



// scripts

#[test]
fn east_asian_width() {
    test_int("./tests/data/eawidth.txt",
        |cp| cp.east_asian_width() as u32);
}

#[test]
fn hangul_syllable_type() {
    test_oint("./tests/data/hangulst.txt",
        |cp| cp.hangul_syllable_type().map(|c| c as u32));
}

#[test]
fn jamo_short_name() { test("./tests/data/jsn.txt", |i, cp, s2| {
    let s1 = cp.jamo_short_name().unwrap_or("-");
    if s1 != s2 { panic!("{}: {:?} {:?}", i, s1, s2); }
});}

#[test]
fn indic_positional_category() {
    test_oint("./tests/data/indic-poscat.txt",
        |cp| cp.indic_positional_category().map(|c| c as u32));
}

#[test]
fn indic_syllabic_category() {
    test_int("./tests/data/indic-sylcat.txt",
        |cp| cp.indic_syllabic_category() as u32);
}

#[test]
fn is_ideograph() {
    test_bool("./tests/data/ideo.txt",
        |cp| cp.is_ideograph());
}

#[test]
fn is_ideograph_description_sequence_binary_operator() {
    test_bool("./tests/data/ideo-desc-seq-bin-op.txt",
        |cp| cp.is_ideograph_description_sequence_binary_operator());
}

#[test]
fn is_ideograph_description_sequence_radical() {
    test_bool("./tests/data/ideo-desc-seq-radical.txt",
        |cp| cp.is_ideograph_description_sequence_radical());
}

#[test]
fn is_ideograph_description_sequence_trinary_operator() {
    test_bool("./tests/data/ideo-desc-seq-trin-op.txt",
        |cp| cp.is_ideograph_description_sequence_trinary_operator());
}

#[test]
fn is_ideograph_unified() {
    test_bool("./tests/data/ideo-unified.txt",
        |cp| cp.is_ideograph_unified());
}

#[test]
fn join_control() {
    test_bool("./tests/data/joinctl.txt",
        |cp| cp.join_control());
}

#[test]
fn joining_group() {
    test_int("./tests/data/joingrp.txt",
        |cp| cp.joining_group() as u32);
}

#[test]
fn joining_type() {
    test_int("./tests/data/jointyp.txt",
        |cp| cp.joining_type() as u32);
}

#[test]
fn script() {
    test_oint("./tests/data/script.txt",
        |cp| cp.script().map(|c| c as u32));
}

#[test]
fn script_extensions() { test("./tests/data/scrext.txt", |i, cp, line| {
    let x1 = cp.script_extensions().map(|s| s.iter().map(|sc| *sc as u32).collect::<Vec<u32>>());
    let x2: Option<Vec<u32>> = if line.len() > 0 {
        Some(line.split(' ').map(|v| v.parse::<u32>().unwrap()).collect())
    } else {
        None
    };

    if x1 != x2 { panic!("{}: {:?} {:?}", i, x1, x2); }
});}



// bidirectionality

#[test]
fn bidi_class() {
    test_int("./tests/data/bidi-class.txt",
        |cp| cp.bidi_class() as u32);
}

#[test]
fn bidi_is_control() {
    test_bool("./tests/data/bidi-control.txt",
        |cp| cp.bidi_is_control());
}

#[test]
fn bidi_is_mirrored() {
    test_bool("./tests/data/bidi-mirrored.txt",
        |cp| cp.bidi_is_mirrored());
}

#[test]
fn bidi_mirror() {
    test_oint("./tests/data/bidi-mirror.txt",
        |cp| cp.bidi_mirror().map(|c| c as u32));
}

#[test]
fn bidi_paired_bracket() {
    test_int("./tests/data/bidi-paired.txt",
        |cp| cp.bidi_paired_bracket() as u32);
}

#[test]
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



// case

#[test]
fn casefold() {
    test_cps("./tests/data/cf.txt",
        |cp| cp.casefold());
}

#[test]
fn casefold_nfkc() {
    test_cps("./tests/data/cf-nfkc.txt",
        |cp| cp.casefold_nfkc());
}

#[test]
fn casefold_nfkc_closure() {
    test_cps("./tests/data/cf-closure.txt",
        |cp| cp.casefold_nfkc_closure());
}

#[test]
fn casefold_simple() {
    test_int("./tests/data/scf.txt",
        |cp| cp.casefold_simple() as u32);
}

#[test]
fn changes_when_casefolded() {
    test_bool("./tests/data/case-changes-casefold.txt",
        |cp| cp.changes_when_casefolded());
}

#[test]
fn changes_when_casefolded_nfkc() {
    test_bool("./tests/data/case-changes-casefold-nfkc.txt",
        |cp| cp.changes_when_casefolded_nfkc());
}

#[test]
fn changes_when_casemapped() {
    test_bool("./tests/data/case-changes-casemap.txt",
        |cp| cp.changes_when_casemapped());
}

#[test]
fn changes_when_lowercased() {
    test_bool("./tests/data/case-changes-lower.txt",
        |cp| cp.changes_when_lowercased());
}

#[test]
fn changes_when_titlecased() {
    test_bool("./tests/data/case-changes-title.txt",
        |cp| cp.changes_when_titlecased());
}

#[test]
fn changes_when_uppercased() {
    test_bool("./tests/data/case-changes-upper.txt",
        |cp| cp.changes_when_uppercased());
}

#[test]
fn is_case_ignorable() {
    test_bool("./tests/data/case-ignorable.txt",
        |cp| cp.is_case_ignorable());
}

#[test]
fn is_cased() {
    test_bool("./tests/data/cased.txt",
        |cp| cp.is_cased());
}

#[test]
fn is_lowercase() {
    test_bool("./tests/data/case-is-lower.txt",
        |cp| Codepoint::is_lowercase(cp));
}

#[test]
fn is_lowercase_other() {
    test_bool("./tests/data/case-is-lower-other.txt",
        |cp| cp.is_lowercase_other());
}

#[test]
fn is_uppercase() {
    test_bool("./tests/data/case-is-upper.txt",
        |cp| Codepoint::is_uppercase(cp));
}

#[test]
fn is_uppercase_other() {
    test_bool("./tests/data/case-is-upper-other.txt",
        |cp| cp.is_uppercase_other());
}

#[test]
fn lowercase() {
    test_cps("./tests/data/lc.txt",
        |cp| cp.lowercase());
}

#[test]
fn lowercase_simple() {
    test_int("./tests/data/slc.txt",
        |cp| cp.lowercase_simple() as u32);
}

#[test]
fn titlecase() {
    test_cps("./tests/data/tc.txt",
        |cp| cp.titlecase());
}

#[test]
fn titlecase_simple() {
    test_int("./tests/data/stc.txt",
        |cp| cp.titlecase_simple() as u32);
}

#[test]
fn uppercase() {
    test_cps("./tests/data/uc.txt",
        |cp| cp.uppercase());
}

#[test]
fn uppercase_simple() {
    test_int("./tests/data/suc.txt",
        |cp| cp.uppercase_simple() as u32);
}



// normalisation

#[test]
fn canonical_combining_class() {
    test_int("./tests/data/ccc.txt",
        |cp| cp.canonical_combining_class() as u32);
}

#[test]
fn decomposition_map() {
    test_cps("./tests/data/dmap.txt",
        |cp| cp.decomposition_map());
}

#[test]
fn decomposition_type() {
    test_oint("./tests/data/dtype.txt",
        |cp| cp.decomposition_type().map(|t| t as u32));
}

#[test]
fn excluded_from_composition() {
    test_bool("./tests/data/comp-excl.txt",
        |cp| cp.excluded_from_composition());
}

#[test]
fn excluded_from_composition_fully() {
    test_bool("./tests/data/comp-excl-full.txt",
        |cp| cp.excluded_from_composition_fully());
}

#[test]
fn expands_on_nfc() {
    test_bool("./tests/data/expanding-nfc.txt",
        |cp| cp.expands_on_nfc());
}

#[test]
fn expands_on_nfd() {
    test_bool("./tests/data/expanding-nfd.txt",
        |cp| cp.expands_on_nfd());
}

#[test]
fn expands_on_nfkc() {
    test_bool("./tests/data/expanding-nfkc.txt",
        |cp| cp.expands_on_nfkc());
}

#[test]
fn expands_on_nfkd() {
    test_bool("./tests/data/expanding-nfkd.txt",
        |cp| cp.expands_on_nfkd());
}

#[test]
fn quick_check_nfc() {
    test_int("./tests/data/qnfc.txt",
        |cp| cp.quick_check_nfc() as u32);
}

#[test]
fn quick_check_nfd() {
    test_bool("./tests/data/quick-nfd.txt",
        |cp| cp.quick_check_nfd());
}

#[test]
fn quick_check_nfkc() {
    test_int("./tests/data/qnfkc.txt",
        |cp| cp.quick_check_nfkc() as u32);
}

#[test]
fn quick_check_nfkd() {
    test_bool("./tests/data/quick-nfkd.txt",
        |cp| cp.quick_check_nfkd());
}



// segmentation

#[test]
fn grapheme_cluster_break() {
    test_int("./tests/data/gcb.txt",
        |cp| cp.grapheme_cluster_break() as u32);
}

#[test]
fn is_grapheme_base() {
    test_bool("./tests/data/graph-base.txt",
        |cp| cp.is_grapheme_base());
}

#[test]
fn is_grapheme_extend() {
    test_bool("./tests/data/graph-ext.txt",
        |cp| cp.is_grapheme_extend());
}

#[test]
fn is_grapheme_extend_other() {
    test_bool("./tests/data/graph-ext-other.txt",
        |cp| cp.is_grapheme_extend_other());
}

#[test]
fn is_grapheme_link() {
    test_bool("./tests/data/graph-link.txt",
        |cp| cp.is_grapheme_link());
}

#[test]
fn linebreak() {
    test_oint("./tests/data/linebreak.txt",
        |cp| cp.linebreak_class().map(|c| c as u32));
}

#[test]
fn sentence_break() {
    test_int("./tests/data/sbrk.txt",
        |cp| cp.sentence_break() as u32);
}

#[test]
fn word_break() {
    test_int("./tests/data/wbrk.txt",
        |cp| cp.word_break() as u32);
}