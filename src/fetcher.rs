use std::io::Read;
use anyhow::{Context, Result, anyhow};
use std::time::Duration; 
use reqwest::blocking::Client; 

pub fn query_celestrak(query: &str, value: &str) -> Result<String> {
    let url: String = format!("https://celestrak.org/NORAD/elements/gp.php?{}={}&FORMAT=tle", query, value);

    // Build a client with a 10s timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .context("Failed to build the reqwest client")?;

    // Make request with client
    let mut response = client.get(&url)
        .send()
        .context("Failed to get a response from celestrak (it may have timed out)")?;

    let mut body = String::new();

    response
        .read_to_string(&mut body)
        .context("Failed to read response to string.")?;

    if !is_query_valid(&body) {
        return Err(anyhow!("Invalid query: {}={}&FORMAT=tle", query, value));
    }

    return Ok(body);
}

fn is_query_valid(response_body: &str) -> bool {
    // Returns false if response_body starts with "Invalid query"
    return !response_body.starts_with("Invalid query");
}


// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_query_valid_with_invalid() {
        let response = "Invalid query: something went wrong";
        assert!(!is_query_valid(response));
    }

    #[test]
    fn test_is_query_valid_with_valid() {
        let response = "ISS (ZARYA)\n1 25544U 98067A   24357.51782407  .00017360  00000-0  31284-3 0  9992";
        assert!(is_query_valid(response));
    }

    // These tests make a network call
    #[test]
    fn test_query_celestrak_valid_query() {
        let result = query_celestrak("CATNR", "25544"); // Query for ISS TLE
        assert!(result.is_ok(), "Expected a valid TLE, got error: {:?}", result.err());
    }

    #[test]
    fn test_query_celestrak_invalid_query() {
        let result = query_celestrak("CATNR", "invalid_id");
        assert!(result.is_err(), "Expected invalid query error");
    }

}
