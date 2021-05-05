use rust_icu::{brk, loc, string, sys};
use std::convert::TryFrom;
use std::fs;
use std::time::Instant;

fn test(locale: &str, test_data_path: &str) {
    let str_utf8 = fs::read_to_string(test_data_path).expect("Loading file from data directory");
    let locale = loc::ULoc::try_from(locale).unwrap();
    let str_utf16 = string::UChar::try_from(str_utf8.as_str()).unwrap();

    let now = Instant::now();
    let iter = brk::UBreakIterator::try_new_ustring(
        sys::UBreakIteratorType::UBRK_LINE,
        &locale,
        &str_utf16,
    )
    .unwrap();
    let diff = now.elapsed().as_micros();
    println!("Initialize line breaker: {}ms", diff);

    let now = Instant::now();
    let _breaks: Vec<_> = iter.collect();
    let diff = now.elapsed().as_micros();
    //println!("Line break opportunities: {:?}", _breaks);
    println!("Iterate line break opportunities: {}ms", diff);
}

fn main() {
    println!("Testing zhuangzi-en.txt");
    test("en", "../../data/zhuangzi-en.txt");

    println!("Testing zhuangzi-zh.txt");
    test("zh", "../../data/zhuangzi-zh.txt");

    println!("Testing thai.txt");
    test("th", "../../data/thai.txt");

    println!();
}
