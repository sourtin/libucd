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
fn block() { test("./tests/data/block.txt", |i, cp, line| {
    let b1 = match cp.block() {
        None => None,
        Some(blk) => Some(blk as u32)
    };

    let b2 = if line.chars().next() == Some('-') {
        None
    } else {
        Some(line.parse::<u32>().unwrap())
    };

    if b1 != b2 { panic!("{}: {:?} {:?}", i, b1, b2); }
});}

#[test]
fn category() { test("./tests/data/cat.txt", |i, cp, line| {
    let c1 = cp.category() as u32;
    let c2 = line.parse::<u32>().unwrap();
    if c1 != c2 { panic!("{}: {:?} {:?}", i, c1, c2); }
});}

#[test] fn combining_class() { test_int("./tests/data/ccc.txt", |cp| cp.combining_class() as u32); }
#[test] fn bidi_control() { test_bool("./tests/data/bidi-control.txt", |cp| cp.bidi_control()); }
#[test] fn bidi_class() { test_int("./tests/data/bidi-class.txt", |cp| cp.bidi_class() as u32); }
#[test] fn bidi_mirrored() { test_bool("./tests/data/bidi-mirrored.txt", |cp| cp.bidi_mirrored()); }
#[test] fn bidi_mirror() { test_oint("./tests/data/bidi-mirror.txt", |cp| cp.bidi_mirror().map(|c| c as u32)); }
#[test] fn bidi_paired() { test_int("./tests/data/bidi-paired.txt", |cp| cp.bidi_paired_bracket() as u32); }

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
}); }
