pub fn align(value: f64, step: f64) -> f64 {
  value - (value % step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align() {
        assert_eq!(align(1.53, 0.5), 1.5);
        assert_eq!(align(421.3, 1.0), 421.0);
        assert_eq!(align(64.6, 15.0), 60.0);
    }
}
