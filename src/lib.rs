//! # Humanize Bytes
//!  
//! Format a number of bytes as a human-readable string.
//! 
//! See for more info: <https://en.wikipedia.org/wiki/Binary_prefix>
//! 
//! 1 KB = 1000 B
//! 
//! 1 KiB = 1024 B
//! 
//! ```rust
//! use humanize_bytes::{humanize_bytes_decimal, humanize_bytes_binary};
//! 
//! println!("{}", humanize_bytes_binary!(0)); // 0 B
//! println!("{}", humanize_bytes_binary!(512)); // 512 B
//! println!("{}", humanize_bytes_binary!(1023)); // 1023 B
//! println!("{}", humanize_bytes_binary!(1024)); // 1 KiB
//! println!("{}", humanize_bytes_binary!(1024 + 99)); // 1 KiB
//! println!("{}", humanize_bytes_binary!(1024 + 103)); // 1.1 KiB
//! println!("{}", humanize_bytes_binary!(1024 * 1024 - 1)); // 1023.9 kB
//! println!("{}", humanize_bytes_binary!(1024 * 1024)); // 1 MB
//! 
//! println!("{}", humanize_bytes_decimal!(0)); // 0 B
//! println!("{}", humanize_bytes_decimal!(512)); // 512 B
//! println!("{}", humanize_bytes_decimal!(999)); // 999 B
//! println!("{}", humanize_bytes_decimal!(1000)); // 1 kB
//! println!("{}", humanize_bytes_decimal!(1000 + 99)); // 1 kB
//! println!("{}", humanize_bytes_decimal!(1000 + 100)); // 1.1 kB
//! println!("{}", humanize_bytes_decimal!(1000 * 1000 - 1)); // 999.9 kB
//! println!("{}", humanize_bytes_decimal!(1000 * 1000)); // 1 MB
//! ```
//!

#[cfg(feature = "binary")]
mod binary {

    ///
    /// Format a number of bytes as a human-readable string, using the SI binary suffixes.
    ///
    /// 1024 (B, KiB, MiB, GiB, TiB, PiB, EiB, ZiB, YiB, RiB, QiB)
    /// 
    #[macro_export]
    macro_rules! humanize_bytes_binary {
        ($value:expr) => {
            {
                use smartstring::{SmartString, LazyCompact};
                use core::fmt::Write;
                let num_bytes = { $value } as f64;
                if num_bytes <= 0.0 {
                    "0 B".into()
                } else if num_bytes < 1024.0 {
                    let mut result = SmartString::<LazyCompact>::new();
                    write!(result, "{} B", num_bytes as u16).unwrap();
                    result
                } else {
                    const SUFFIX: [&str; 11] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB", "RiB", "QiB"];
                    const UNIT: f64 = 1024.0;
                    let base = num_bytes.log2() as usize / 10;
                    let curr_base = UNIT.powi(base as i32) as f64;
                    let units = num_bytes / curr_base;
                    let units = (units * 100.0).floor() / 100.0;
                    let mut once = true;
                    let mut extra = SmartString::<LazyCompact>::new();
                    write!(extra, "{:.2}", units).unwrap();
                    let trimmed = extra 
                        .trim_end_matches(|_| if once { once = false; true } else { false })
                        .trim_end_matches("0")
                        .trim_end_matches(".");
                    let mut result: SmartString<LazyCompact> = trimmed.into();
                    result.push_str(" ");
                    result.push_str(SUFFIX[base as usize]);
                    result
                }
            }
        }
    }

    pub use humanize_bytes_binary;
}

#[cfg(feature = "decimal")]
mod decimal {

    ///
    /// Format a number of bytes as a human-readable string, using the IEC decimal suffixes.
    ///
    /// 1000 (B, kB, MB, GB, TB, PB, EB, ZB, YB) 
    #[macro_export]
    macro_rules! humanize_bytes_decimal {
        ($value:expr) => {
            {
                use smartstring::{SmartString, LazyCompact};
                use core::fmt::Write;
                let num_bytes = { $value } as f64;
                if num_bytes <= 0.0 {
                    "0 B".into()
                } else if num_bytes < 1000.0 {
                    let mut result = SmartString::<LazyCompact>::new();
                    write!(result, "{} B", num_bytes as u16).unwrap();
                    result
                } else {
                    const SUFFIX: [&str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
                    const UNIT: f64 = 1000.0;
                    let base = num_bytes.log10() as usize / 3;
                    let curr_base = UNIT.powi(base as i32) as f64;
                    let units = num_bytes / curr_base;
                    let units = (units * 100.0).floor() / 100.0;
                    let mut once = true;
                    let mut extra = SmartString::<LazyCompact>::new();
                    write!(extra, "{:.2}", units).unwrap();
                    let trimmed = extra 
                        .trim_end_matches(|_| if once { once = false; true } else { false })
                        .trim_end_matches("0")
                        .trim_end_matches(".");
                    let mut result: SmartString<LazyCompact> = trimmed.into();
                    result.push_str(" ");
                    result.push_str(SUFFIX[base as usize]);
                    result                
                }
            }        
        }
    }

    pub use humanize_bytes_decimal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanize_bytes_binary() {
        assert_eq!(humanize_bytes_binary!(0), "0 B");
        assert_eq!(humanize_bytes_binary!(1), "1 B");
        assert_eq!(humanize_bytes_binary!(512), "512 B");
        assert_eq!(humanize_bytes_binary!(1023), "1023 B");
        assert_eq!(humanize_bytes_binary!(1024), "1 KiB");
        assert_eq!(humanize_bytes_binary!(1025), "1 KiB");
        assert_eq!(humanize_bytes_binary!(1024 + 99), "1 KiB");
        assert_eq!(humanize_bytes_binary!(1024 + 103), "1.1 KiB");
        assert_eq!(humanize_bytes_binary!(1024 * 1024 - 1), "1023.9 KiB");
        assert_eq!(humanize_bytes_binary!(1024 * 1024), "1 MiB");
        assert_eq!(humanize_bytes_binary!(1024 * 1024 * 1024 - 1), "1023.9 MiB");
        assert_eq!(humanize_bytes_binary!(1024 * 1024 * 1024), "1 GiB");
        assert_eq!(humanize_bytes_binary!(1024u64 * 1024 * 1024 * 1024), "1 TiB");
        assert_eq!(humanize_bytes_binary!(1024u64 * 1024 * 1024 * 1024 * 1024), "1 PiB");
        assert_eq!(humanize_bytes_binary!(1024u128 * 1024 * 1024 * 1024 * 1024 * 1024), "1 EiB");
        assert_eq!(humanize_bytes_binary!(1024u128 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1 ZiB");
        assert_eq!(humanize_bytes_binary!(1024u128 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1 YiB");
        assert_eq!(humanize_bytes_binary!(1024u128 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1 RiB");
        assert_eq!(humanize_bytes_binary!(1024u128 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1 QiB");
    }

    #[test]
    fn test_humanize_bytes_decimal() {
        assert_eq!(humanize_bytes_decimal!(0), "0 B");
        assert_eq!(humanize_bytes_decimal!(1), "1 B");
        assert_eq!(humanize_bytes_decimal!(512), "512 B");
        assert_eq!(humanize_bytes_decimal!(999), "999 B");
        assert_eq!(humanize_bytes_decimal!(1000), "1 kB");
        assert_eq!(humanize_bytes_decimal!(1099), "1 kB");
        assert_eq!(humanize_bytes_decimal!(1100), "1.1 kB");
        assert_eq!(humanize_bytes_decimal!(999_999), "999.9 kB");
        assert_eq!(humanize_bytes_decimal!(1_000_000), "1 MB");
        assert_eq!(humanize_bytes_decimal!(999_999_999), "999.9 MB");
        assert_eq!(humanize_bytes_decimal!(1_000_000_000), "1 GB");
        assert_eq!(humanize_bytes_decimal!(1_000_000_000_000u64), "1 TB");
        assert_eq!(humanize_bytes_decimal!(1_000_000_000_000_000u64), "1 PB");
        assert_eq!(humanize_bytes_decimal!(1_000_000_000_000_000_000u128), "1 EB");
        assert_eq!(humanize_bytes_decimal!(1_000_000_000_000_000_000_000u128), "1 ZB");
        assert_eq!(humanize_bytes_decimal!(1_000_000_000_000_000_000_000_000u128), "1 YB");
    }
}

