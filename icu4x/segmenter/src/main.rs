use icu_segmenter::LineBreakIteratorUtf16;
use std::fs;
use std::time::Instant;

fn test(test_data_path: &str) {
    let str_utf8 = fs::read_to_string(test_data_path).expect("Loading file from data directory");
    let str_utf16: Vec<u16> = str_utf8.encode_utf16().collect();

    let now = Instant::now();
    let iter = LineBreakIteratorUtf16::new(&str_utf16);
    let diff = now.elapsed().as_micros();
    println!("Initialize line breaker: {}ms", diff);

    let now = Instant::now();
    let _breaks: Vec<usize> = iter.collect();
    let diff = now.elapsed().as_micros();
    //println!("Line break opportunities: {:?}", _breaks);
    println!("Iterate line break opportunities: {}ms", diff);
}

fn main() {
    println!("Testing zhuangzi-en.txt");
    test("../../data/zhuangzi-en.txt");

    println!("Testing zhuangzi-zh.txt");
    test("../../data/zhuangzi-zh.txt");

    println!("Testing thai.txt");
    test("../../data/thai.txt");

    println!();
}
