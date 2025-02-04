use rust_journey_2025::day1_rust_foundation::{functions, matches};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(functions::max(5, 10), 10);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(functions::fibonacci(5), 5);
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(matches::day_of_week(1), "Monday")
    }
 }