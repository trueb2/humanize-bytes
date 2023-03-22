#[cfg(test)]
mod tests {
    use humanize_bytes::{humanize_bytes_binary, humanize_bytes_decimal, humanize_quantity};

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
        assert_eq!(humanize_bytes_decimal!(1_000_000_000_000_000_000_000_000_000u128), "1 RB");
        assert_eq!(humanize_bytes_decimal!(1_000_000_000_000_000_000_000_000_000_000u128), "1 QB");
    }

    #[test]
    fn test_humanize_quantity() {
        assert_eq!(humanize_quantity!(0), "0");
        assert_eq!(humanize_quantity!(1), "1");
        assert_eq!(humanize_quantity!(512), "512");
        assert_eq!(humanize_quantity!(999), "999");
        assert_eq!(humanize_quantity!(1000), "1 k");
        assert_eq!(humanize_quantity!(1099), "1 k");
        assert_eq!(humanize_quantity!(1100), "1.1 k");
        assert_eq!(humanize_quantity!(999_999), "999.9 k");
        assert_eq!(humanize_quantity!(1_000_000), "1 M");
        assert_eq!(humanize_quantity!(999_999_999), "999.9 M");
        assert_eq!(humanize_quantity!(1_000_000_000), "1 G");
        assert_eq!(humanize_quantity!(1_000_000_000_000u64), "1 T");
        assert_eq!(humanize_quantity!(1_000_000_000_000_000u64), "1 P");
        assert_eq!(humanize_quantity!(1_000_000_000_000_000_000u128), "1 E");
        assert_eq!(humanize_quantity!(1_000_000_000_000_000_000_000u128), "1 Z");
        assert_eq!(humanize_quantity!(1_000_000_000_000_000_000_000_000u128), "1 Y");
        assert_eq!(humanize_quantity!(1_000_000_000_000_000_000_000_000_000u128), "1 R");
        assert_eq!(humanize_quantity!(1_000_000_000_000_000_000_000_000_000_000u128), "1 Q");
    }
}