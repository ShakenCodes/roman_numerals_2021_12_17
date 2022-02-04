pub fn to_roman(arabic: u32) -> String {
    let mut remainder = arabic;
    let mut result = String::new();
    while remainder >= 1000 {
        result.push('M');
        remainder -= 1000;
    }
    if remainder >= 900 {
        result.push_str("CM");
        remainder -= 900;
    }
    if remainder >= 500 {
        result.push_str("D");
        remainder -= 500;
    }
    if remainder >= 400 {
        result.push_str("CD");
        remainder -= 400;
    }
    while remainder >= 100 {
        result.push('C');
        remainder -= 100;
    }

    result
}