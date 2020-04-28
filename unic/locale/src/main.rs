use std::time::Instant;
use std::mem;
use locale::STRINGS;
use unic_langid::LanguageIdentifier;
use std::str::FromStr;

fn main() {

    {
        // Construct
        let now = Instant::now();

        for langid in STRINGS {
            let _ = LanguageIdentifier::from_str(langid);
        }

        let measured_us = now.elapsed().as_micros();
        println!("Create Locale from str for {} locales. time: {} us", STRINGS.len(), measured_us);
    }

    let locales: Vec<LanguageIdentifier> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

    {
        let reference_locale = LanguageIdentifier::from_str("en-US").unwrap();

        let mut matches = 0;

        let now = Instant::now();

        for loc in &locales {
            if *loc == reference_locale {
                matches += 1;
            }
        }

        let measured_ns = now.elapsed().as_nanos();
        println!("Number of matches against en-US: {}. time: {} ns", matches, measured_ns);
    }

    {
        // Size

        // XXX: This does not measure heap allocation, leaving `variants` not accounted for.
        // Since there are only 3 locales with variants in the sample, we hope the results
        // remain meaningful, but it would be nice to improve the measuring.
        let size = mem::size_of::<LanguageIdentifier>() * locales.capacity();
        println!("Total size of the locales vector: {} bytes.", size);
    }

    {
        // ToString
        let now = Instant::now();

        for loc in &locales {
            let _ = loc.to_string();
        }

        let measured_us = now.elapsed().as_micros();
        println!("Serialized Locale. time: {} us", measured_us);
    }

    {
        // {Add|Remove}LikelySubtags
        let now = Instant::now();

        for mut loc in locales {
            loc.maximize();
        }

        let measured_us = now.elapsed().as_micros();
        println!("Added/Removed likely subtags. time: {} us", measured_us);
    }
}
