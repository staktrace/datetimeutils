systimeutils
============

A handful of utility functions for dealing with `std::time::SystemTime` in somewhat-useful ways.
If you need handling for timezones, look elsewhere. This crate only deals with simple stuff.
The main attraction is that it has zero external dependencies, so if it does what you want, it's lighter-weight than using `chrono` or `time`.
