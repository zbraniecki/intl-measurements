use std::mem;
use locale::STRINGS;
use unic_langid::LanguageIdentifier;

fn main() {
    let locales: Vec<LanguageIdentifier> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();
    println!("Size: {} bytes", mem::size_of::<LanguageIdentifier>() * locales.capacity());
}
