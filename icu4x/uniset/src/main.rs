use icu_unicodeset::UnicodeSet;
use std::char;
use std::time::Instant;

fn main() {
    let best_ex = vec!['A'.into(), 'F'.into()];
    let best_sample = UnicodeSet::from_inversion_list(best_ex).unwrap();
    let worst_ex: Vec<u32> = (0..((char::MAX as u32) + 1)).collect();
    let worst_sample = UnicodeSet::from_inversion_list(worst_ex).unwrap();

    let zero = char::from_u32(0).unwrap();

    let mut total = 0;

    let now = Instant::now();

    for c in best_sample.iter() {
        if best_sample.contains(c) {
            total += 1;
        }
    }

    for c in worst_sample.iter().take(100) {
        if worst_sample.contains(c) {
            total += 1;
        }
    }

    for c in best_sample.iter() {
        if best_sample.contains_range(&('A'..=c)) {
            total += 1;
        }
    }

    for c in worst_sample.iter().take(100) {
        //XXX: This test differs from ICU4X benchmark because
        // I couldn't get an equivalent of `contains_range(&(0..0))`
        // in C++.
        if worst_sample.contains_range(&(zero..=c)) {
            total += 1;
        }
    }

    let end = now.elapsed().as_nanos();

    println!("UnicodeSet: {} ns, total: {}", end, total);
}
