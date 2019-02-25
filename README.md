# librtlsdr rust bindings
This crate provides bindings for `librtlsdr` for linkage. Following the `-sys` convention it does not provide at "rusty" API on top - just pure C bindings.

## Usage
Add `librtlsdr-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
librtlsdr-sys = "0.1"
```

Import the `librtlsdr_sys` crate and use the functions as they're defined in the native `librtlsdr` library. See additional documentation below for more information.

Rust pre-2018
```rust
extern crate librtlsdr_sys;
```

Rust 2018
```rust
use librtlsdr_sys;
```

### Help
Since this crate does not provide anything more than what is available in `librtlsdr`. The best place to look for help is:

* [rtlsdr wiki](https://osmocom.org/projects/rtl-sdr/wiki)
* [rtlsdr github](https://github.com/osmocom/rtl-sdr)

## License
Copyright © 2019 Bo Stendal Sorensen
