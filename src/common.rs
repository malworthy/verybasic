pub fn round(number: f64, decimal_places: u32) -> f64 {
    let y = 10i32.pow(decimal_places) as f64;
    (number * y).round() / y
}

#[cfg(test)]
mod tests {
    use super::round;

    #[test]
    fn test_round() {
        assert_eq!(round(123.45999, 2), 123.46);
        assert_eq!(round(123.454, 2), 123.45);
        assert_eq!(round(123.455, 2), 123.46);
    }
}
