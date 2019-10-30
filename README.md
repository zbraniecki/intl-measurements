# intl-measurements
Measurements of various intl approaches.

This is a very hacky work-in-progress at the moment. I'll try to accumulate test results between ICU (C++), MozLocale (C++) and Rust `unic-locale` & friends.

I'll keep this doc with instructions.

# Current results (as of Oct 30 2019)

Spec: MacBook Pro 2017, 3.1 GHz Quad-Core Intel Core i7, 16 GB RAM, MacOS 10.15

* **C++**: GCC 9.2
* **Rust**: 1.28
* **ICU**: 64
* **Unic**: intl-locale 0.7

Sample: 956 locale strings provided to MozLocale constructor during fresh-profile startup of Firefox Nightly on Oct 21st 2019.


|                    Test                      |   ICU     |    UNIC   |  Difference |
| -------------------------------------------: | --------: | --------: | ----------: |
| Construct an instance from a string          |    258 us |  26.48 us |     -89.74% |
| Matching all locales against `en-US`         |     11 us |  1.239 us |     -88.74% |
| Serializing all locales to a string          |    970 us | 117.89 us |     -87.85% |
| Adding/Removing likely subtags               |   4589 us |  56.06 us |     -98.78% |
| Measuring memory allocation of all instances |  229376 b |   30592 b |     -86.66% |


# How to run

1) Make sure you have ICU4C installed (some additional settings for homebrew for mac in `icu/macos.txt`)
2) `cd ./icu`
3) `make`
4) `./locale`

5) `cd  ../unic/locale`
6) `cargo bench`  - for perf benchmarks
7) `cargo run` - for memory size read

# Limitations

Currently the memory read is suboptimal, but based on review by Adam Gashlin, should be fairly accurate for both C++ and Rust minus `variants`. Since there are only 3 variants in a pool of close to 1000 locales to sample, the results should be fairly accurate.

Since we use benchmark suite for Rust, we don't really have a good way to measure "just adding likelys subtags" since that would require constructing a new set of `LanguageIdentifier` instances per iteration and would be hard to exclude from measurement.
Instead, we construct maximized instances and then cycle `minimize/maximize` per iteration. This should give us fairly good approximation of the results and comparable between ICU and Rust.
