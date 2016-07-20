extern crate ucd;
use ucd::Codepoint;
use std::char;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

//pub mod tables;

#[test]
fn age() {
    let f = File::open("./tests/data/age.txt").unwrap();
    let reader = BufReader::new(f);

    let mut max: usize = 0;
    for (i, line) in reader.lines().enumerate() {
        max = i;
        let chars = line.unwrap().chars().collect::<Vec<char>>();

        assert!(i < 0x110000);
        let c = unsafe { char::from_u32_unchecked(i as u32) };
        let cp = Codepoint(c);

        let a1 = cp.age();
        let a2 = if chars[0] == '-' {
            None
        } else {
            Some((chars[0].to_digit(10).unwrap() as u8,
                  chars[1].to_digit(10).unwrap() as u8))
        };

        if a1 != a2 {
            panic!("{}: {:?} {:?}", i, a1, a2);
        }
    }

    assert!(max == 0x10ffff);
}
