use chrono::NaiveDate;

pub fn parse_iso8601_date(input: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date '{}': {}", input, e))
}
