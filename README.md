datetimeutils
=============
![Build Status](https://github.com/staktrace/datetimeutils/actions/workflows/test.yml/badge.svg)
[![Crate](https://img.shields.io/crates/v/datetimeutils.svg)](https://crates.io/crates/datetimeutils)

A handful of utility functions for dealing with `std::time::SystemTime` in somewhat-useful ways.
If you need handling for timezones, look elsewhere. This crate only deals with simple stuff.
The main attraction is that it has zero external dependencies, so if it does what you want, it's lighter-weight than using `chrono` or `time`.

API
---
There's a bunch of public functions in the crate to do basic datetime stuff such as find the number of days in a particular year or index months.
Also there's a struct `PostEpochTime` which effectively wraps a `std::time::SystemTime` but can provide human-readable date/time values from it.
See full rustdoc at [https://docs.rs/datetimeutils](https://docs.rs/datetimeutils).

Examples
--------
See files in the `examples/` folder for quick examples.
