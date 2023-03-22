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
//! use humanize_bytes::{humanize_bytes_decimal, humanize_bytes_binary, humanize_quantity};
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
//! 
//! println!("{}", humanize_quantity!(0)); // 0
//! println!("{}", humanize_quantity!(512)); // 512
//! println!("{}", humanize_quantity!(999)); // 999
//! println!("{}", humanize_quantity!(1000)); // 1 k
//! println!("{}", humanize_quantity!(1000 + 99)); // 1 k
//! println!("{}", humanize_quantity!(1000 + 100)); // 1.1 k
//! println!("{}", humanize_quantity!(1000 * 1000 - 1)); // 999.9 k
//! println!("{}", humanize_quantity!(1000 * 1000)); // 1 M
//! ```
//!

pub use smartstring;

mod binary {

    ///
    /// Format a number of bytes as a human-readable string, using the IEC binary suffixes.
    ///
    /// 1024 (B, KiB, MiB, GiB, TiB, PiB, EiB, ZiB, YiB, RiB, QiB)
    /// 
    #[macro_export]
    macro_rules! humanize_bytes_binary {
        ($value:expr) => {
            {
                use ::humanize_bytes::smartstring::{SmartString, LazyCompact};
                use ::core::fmt::Write;
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

mod decimal {

    ///
    /// Format a number of bytes as a human-readable string, using the SI decimal suffixes.
    ///
    /// 1000 (B, kB, MB, GB, TB, PB, EB, ZB, YB) 
    #[macro_export]
    macro_rules! humanize_bytes_decimal {
        ($value:expr) => {
            {
                use ::humanize_bytes::smartstring::{SmartString, LazyCompact};
                use ::core::fmt::Write;
                let num_bytes = { $value } as f64;
                if num_bytes <= 0.0 {
                    "0 B".into()
                } else if num_bytes < 1000.0 {
                    let mut result = SmartString::<LazyCompact>::new();
                    write!(result, "{} B", num_bytes as u16).unwrap();
                    result
                } else {
                    const SUFFIX: [&str; 11] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "RB", "QB"];
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


    ///
    /// Format a number of bytes as a human-readable string, using the SI decimal suffixes.
    ///
    /// Factors of 1000 (, k, M, G, T, P, E, Z, Y, R, Q) 
    /// kilo, mega, giga, tera, peta, exa, zetta, yotta, ronna, quetta
    /// 
    #[macro_export]
    macro_rules! humanize_quantity {
        ($value:expr) => {
            {
                use ::humanize_bytes::smartstring::{SmartString, LazyCompact};
                use ::core::fmt::Write;
                let num_bytes = { $value } as f64;
                if num_bytes <= 0.0 {
                    "0".into()
                } else if num_bytes < 1000.0 {
                    let mut result = SmartString::<LazyCompact>::new();
                    write!(result, "{}", num_bytes as u16).unwrap();
                    result
                } else {
                    const SUFFIX: [&str; 11] = ["", "k", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"];
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

    pub use humanize_quantity;
}