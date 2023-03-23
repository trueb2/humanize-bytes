[![CI](https://github.com/trueb2/humanize-bytes/actions/workflows/rust.yml/badge.svg)](https://github.com/trueb2/humanize-bytes/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/humanize-bytes)](https://crates.io/crates/humanize-bytes)


# Humanize Bytes

Create a human-readable string representation of a number of bytes either in binary or decimal units (SI or IEC).

https://en.wikipedia.org/wiki/Binary_prefix

1 KB = 1000 B
1 KiB = 1024 B
 
```rust
use humanize_bytes::{humanize_bytes_decimal, humanize_bytes_binary, humanize_quantity};
 
assert_eq!(humanize_bytes_binary!(0), "0 B");
assert_eq!(humanize_bytes_binary!(512), "512 B");
assert_eq!(humanize_bytes_binary!(1023), "1023 B");
assert_eq!(humanize_bytes_binary!(1024), "1 KiB");
assert_eq!(humanize_bytes_binary!(1024 + 99), "1 KiB");
assert_eq!(humanize_bytes_binary!(1024 + 103), "1.1 KiB");
assert_eq!(humanize_bytes_binary!(1024 * 1024 - 1), "1023.9 KiB");
assert_eq!(humanize_bytes_binary!(1024 * 1024), "1 MiB");
assert_eq!(humanize_bytes_binary!(1024 * 1024 * 1024), "1 GiB");

assert_eq!(humanize_bytes_decimal!(0), "0 B");
assert_eq!(humanize_bytes_decimal!(512), "512 B");
assert_eq!(humanize_bytes_decimal!(999), "999 B");
assert_eq!(humanize_bytes_decimal!(1000), "1 kB");
assert_eq!(humanize_bytes_decimal!(1000 + 99), "1 kB");
assert_eq!(humanize_bytes_decimal!(1000 + 100), "1.1 kB");
assert_eq!(humanize_bytes_decimal!(1000 * 1000 - 1), "999.9 kB");
assert_eq!(humanize_bytes_decimal!(1000 * 1000), "1 MB");
assert_eq!(humanize_bytes_decimal!(1000 * 1000 * 1000), "1 GB");

assert_eq!(humanize_quantity!(0), "0");
assert_eq!(humanize_quantity!(512), "512");
assert_eq!(humanize_quantity!(999), "999");
assert_eq!(humanize_quantity!(1000), "1 k");
assert_eq!(humanize_quantity!(1000 + 99), "1 k");
assert_eq!(humanize_quantity!(1000 + 100), "1.1 k");
assert_eq!(humanize_quantity!(1000 * 1000 - 1), "999.9 k");
assert_eq!(humanize_quantity!(1000 * 1000), "1 M");
```

## Implementation Details

This crate has one dependency, `smartstring`, and does not allocate because all formatting fits within the `MAX_INLINE` limit for a `SmartString<LazyCompact>`. Both macros return a `SmartString<LazyCompact>`, which looks/feels just like a normal `String`.
