#[cfg(test)]
mod tests {
    use roman_numerals::to_roman;

    #[test]
    fn test_to_roman() {
        assert_eq!(to_roman(3000), "MMM");
        assert_eq!(to_roman(2500), "MMD");
        assert_eq!(to_roman(1000), "M");
        assert_eq!(to_roman(900), "CM");
        assert_eq!(to_roman(500), "D");
        assert_eq!(to_roman(400), "CD");
        assert_eq!(to_roman(300), "CCC");
        assert_eq!(to_roman(100), "C");
    }
}
