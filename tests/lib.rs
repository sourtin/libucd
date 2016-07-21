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

#[test]
fn combining_class() { test("./tests/data/ccc.txt", |i, cp, line| {
    let c1 = cp.combining_class() as u32;
    let c2 = line.parse::<u32>().unwrap();
    if c1 != c2 { panic!("{}: {:?} {:?}", i, c1, c2); }
});}
