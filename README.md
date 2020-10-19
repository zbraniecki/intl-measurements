# intl-measurements

This repository is a sub-project of ICU4X project and contains a harness and implementations of small benchmarks for measuring performance, memory and size differences between various implementations of ICU.

The main goal of this project is to establish a reasonable framework for comparing those measurements across multiple implementations and validate a claim that ICU4X is not slower, doesn't use more memory and doesn't take more space than a comparable alternatives.

# Tests

At the moment the harness is focused on measuring performance between:

* ICU4X 0.1
* ICU4C 67 (using C++ apps)
* ICU4C 67 (via `rust_icu`)
* ICU4C 67 (via FFI)
* A set of earlier Rust implementations of components such as `intl_pluralrules`, `unic_locale`, and `unic_datetime`

The aim is to make it easy to compare different backends for similar basic operations and minimize noise in data.

The tests can be launched via `stand-alone` binaries, or in case of all Rust apps via `criterion` benchmark.

# Current results (as of October 19 2020)

Specs:
* Dell Tower 5820
  * Intel(R) Xeon(R) W-2155 CPU @ 3.30GHz x20
  * 32 GB RAM

* **C++**: GCC 10
* **Rust**: 1.47
* **ICU4C**: 67
* **ICU4X**: icu4x 0.1

Sample: 956 locale strings provided to MozLocale constructor during fresh-profile startup of Firefox Nightly on Oct 21st 2019.


|                    Test                      | ICU4X | ICU4C (rust_icu) | ICU4X (FFI) | ICU4C (C++) | Unic |
| -------------------------------------------: | --------: | --------: | ----------: | ----------: | ----------: |
| **Locale**  | | | | | |
| Construct an instance from a string          | 28,531 ns | 821,947 ns | 541,333 ns | 1,235,646 ns | 41,578 ns |
| Filter all locales against `en-US`           | 3,035 ns | 5,539 ns | 10,013 ns | 48,195 ns | 3,067 ns |
| Serializing all locales to a string          | 68,072 ns | 77,826 ns | 66,682 ns | 2,982,821 ns | 75,760 ns |
| Canonicalize all strings                     | 88,416 ns | 942,376 ns | 217,938 ns | 2,010,337 ns | 115,324 ns |
| Measuring memory allocation of all instances | 30,592 b | 22,944 b | 91,776 b | 229,376 b | 30,592 b |
| **Plural Rules**  | | | | | |
| Select | 92 ns | 289 ns | 287 ns | 123,822 ns |  5 ns |
