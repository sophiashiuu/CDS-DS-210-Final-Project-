//tstats

pub fn calculate_median(data: &[u32]) -> f32 {
    let n = data.len();
    if n == 0 {
        return 0.0;
    }
    let mid = n / 2;
    if n % 2 == 0 {
        (data[mid - 1] as f32 + data[mid] as f32) / 2.0
    } else {
        data[mid] as f32
    }
 }
 
 
 
 pub fn parse_year_from_date(date: &str) -> u32 {
    date.split('-').next().unwrap_or("0").parse().unwrap_or(0)
 }
 




 
 //test 

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_dates() {
        assert_eq!(parse_year_from_date("2021-12-31"), 2021);
        assert_eq!(parse_year_from_date("1999-01-01"), 1999);
        assert_eq!(parse_year_from_date("2000-06-15"), 2000);
    }

    #[test]
    fn test_parse_invalid_dates() {
        assert_eq!(parse_year_from_date("2021/12/31"), 0); // Incorrect separator
        assert_eq!(parse_year_from_date("December 31, 2021"), 0); // Non-ISO format
        assert_eq!(parse_year_from_date("2021"), 2021); // Year only (valid edge case)
    }

    #[test]
    fn test_parse_edge_cases() {
        assert_eq!(parse_year_from_date(""), 0); // Empty string
        assert_eq!(parse_year_from_date("Some random string"), 0); // Non-date string
        assert_eq!(parse_year_from_date("2021-"), 2021); // Incomplete date but valid year
        assert_eq!(parse_year_from_date("-2021"), 0); // Leading separator
    }
}
