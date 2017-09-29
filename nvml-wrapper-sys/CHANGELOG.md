# nvml-wrapper-sys Changelog

This file describes the changes / additions / fixes between bindings releases.

## Unreleased

Bindings have been regenerated using the NVML 9 header and bindgen 0.30.0.

## 0.3.0 (released 2017-07-20)

### Release Summary

The `nightly` feature flag has been removed as unions are now available on stable Rust.

### Rust Version Support

This release **requires** and supports **Rust 1.19.0** or higher.

## 0.2.0 (released 2017-06-08)

### Release Summary

Rust `enum`s were removed in favor of numerical constants for C enums. This was done for safety reasons; see [rust-lang/rust#36927](https://github.com/rust-lang/rust/issues/36927) for more information.

### Changes

* Rust `enum`s replaced with numerical constants
* Replaced `::std::os::raw::x` paths with `raw::x` paths for readability
* Removed `Copy` and `Clone` from structs where they did not make sense
  * Forgot about this before

## 0.1.0 (released 2017-05-17)

### Release Summary

Initial release providing bindings for the entirety of the NVML API as well as nightly-only feature usage behind a feature flag.
