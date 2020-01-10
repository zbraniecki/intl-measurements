# intl-measurements
Measurements of various intl approaches.

This is a very hacky work-in-progress at the moment. I'll try to accumulate test results between ICU (C++), MozLocale (C++) and Rust `unic-locale` & friends.

I'll keep this doc with instructions.

# Current results (as of January 9 2020)

Spec: Dell XPS 7590, i9-9980HK CPU @ 2.40GHz, 64GB RAM, Linux Arch

* **C++**: GCC 9.2
* **Rust**: 1.40
* **ICU**: 65
* **Unic**: intl-locale 0.7, intl_pluralrules 5.0

Sample: 956 locale strings provided to MozLocale constructor during fresh-profile startup of Firefox Nightly on Oct 21st 2019.


|                    Test                      |   ICU     |    UNIC   |  Difference |
| -------------------------------------------: | --------: | --------: | ----------: |
| **Locale**  | | | |
| Construct an instance from a string          |    113 us |     22 us |     -80.53% |
| Matching all locales against `en-US`         |   4631 ns |   1803 us |     -61.06% |
| Serializing all locales to a string          |    542 us |     83 us |     -84.68% |
| Adding/Removing likely subtags               |   2457 us |     67 us |     -97.27% |
| Measuring memory allocation of all instances |  229376 b |   30592 b |     -86.66% |
| **PluralRules**  | | | |
|Select 31 numbers for 10 locales              | 185629 ns |   3726 ns |     -97.99% |


# How to run

1) Make sure you have ICU4C installed (some additional settings for homebrew for mac in `icu/macos.txt`)
2) `cd ./icu`
3) `make all`
4) `./locale`
5) `./pluralrules`

6) `cd  ../unic/locale`
7) `cargo run --release` - for a single-run measurements (like C++), and memory read
8) `cargo bench`  - for statistically valid perf benchmarks
9) `cd ../pluralrules`
10) `cargo run --release` - for a single-run measurements (like C++)

# Limitations

Currently the memory read is suboptimal, but based on review by Adam Gashlin, should be fairly accurate for both C++ and Rust minus `variants`. Since there are only 3 variants in a pool of close to 1000 locales to sample, the results should be fairly accurate.

Since we use benchmark suite for Rust, we don't really have a good way to measure "just adding likelys subtags" since that would require constructing a new set of `LanguageIdentifier` instances per iteration and would be hard to exclude from measurement.
Instead, we construct maximized instances and then cycle `minimize/maximize` per iteration. This should give us fairly good approximation of the results and comparable between ICU and Rust.

The `cargo run --release` should be a good 1-1 equivalent of the C++ calls.
