use std::mem;
use locale::STRINGS;
use unic_langid::LanguageIdentifier;

fn main() {
    let locales: Vec<LanguageIdentifier> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    // XXX: This does not measure heap allocation, leaving `variants` not accounted for.
    // Since there are only 3 locales with variants in the sample, we hope the results
    // remain meaningful, but it would be nice to improve the measuring.
    println!("Size: {} bytes", mem::size_of::<LanguageIdentifier>() * locales.capacity());
}
