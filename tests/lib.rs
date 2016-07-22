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

#[test] #[ignore] fn bidi_control() { test_bool("./tests/data/bidi-control.txt", |cp| cp.bidi_control()); }
#[test] #[ignore] fn bidi_class() { test_int("./tests/data/bidi-class.txt", |cp| cp.bidi_class() as u32); }
#[test] #[ignore] fn bidi_mirrored() { test_bool("./tests/data/bidi-mirrored.txt", |cp| cp.bidi_mirrored()); }
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
#[test] #[ignore] fn deprecated() { test_bool("./tests/data/deprec.txt", |cp| cp.deprecated()); }
#[test] #[ignore] fn variation_selector() { test_bool("./tests/data/varsel.txt", |cp| cp.variation_selector()); }
#[test] #[ignore] fn noncharacter() { test_bool("./tests/data/nonchar.txt", |cp| cp.noncharacter()); }
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
