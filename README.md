# Crate locale-codes

[![travis.ci](https://travis-ci.org/johnstonskj/locale-codes.svg?branch=master)](https://travis-ci.org/johnstonskj/locale-codes)
[![crates.io](https://img.shields.io/crates/v/locale-codes.svg)](https://crates.io/crates/locale-codes)
[![docs.rs](https://docs.rs/locale-codes/badge.svg)](https://docs.rs/locale-codes)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
![mit License](https://img.shields.io/badge/license-mit-118811.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/locale-codes.svg)](https://github.com/johnstonskj/locale-codes/stargazers)

This crate provides locale-related codes/identifiers and any standards-based information
concerning them. For example, ISO-396 language identifiers, or ISO-3166
country identifiers.

## Example

```rust
use locale_codes::codes::{country, currency, region};

let mexico = country::lookup_country("MEX").unwrap();
println!("{:?}", mexico);

let mexico_region = country::lookup_region(mexico.country_code).unwrap();
println!("{:?}", mexico_region);

let currencies = currency::currencies_for_country_name(mexico_region.name.as_str());
println!("{:?}", currencies);
```

## Pre-Build Process

The following describe two code generation steps that are executed outside
the normal build process as the output is stored in Git and versioned 
based on external factors.

### JSON Data Files

The script [`create-data-modules`](https://github.com/johnstonskj/locale-codes/blob/master/create-data-modules.sh)
on the other hand is used to process files downloaded, or scraped, from
standards web sites to create data used by the library. This data is generated
as JSON files in the `src/codes/data` folder and read as a part of the 
build for `codes` modules using the Rust `include!` macro.

Currently data is generated for the following standards:

* ISO 639 _Codes for the representation of names of languages_; Parts 1-4, 
  2-character and 3-character codes supported. 
* ISO 3166 _Codes for the representation of names of countries and their 
  subdivisions_; Part 1, 2-character codes, only.
* ISO 4217 _Codes for the representation of currencies_; alphabetic and 
  numeric codes supported.
* ISO 15924 _Codes for the representation of names of scripts_; alphabetic 
  and numeric codes supported.

## History

* **0.1.0** - extracted from [simple-locale](https://github.com/johnstonskj/simple-locale).

## TODO

* Determine naming convention between the names in the `codes` and `settings`
  crates. 
  * Expect that the names in code modules will be changed to reflect
    those in the settings.
