use anyhow::Result;
use sgp4::{Prediction};

use crate::tle::TLE;

#[derive(Debug)]
pub struct PropagationResult {
    pub object_name: String,
    pub predictions: Vec<Prediction>
}

// Propagate each fetched TLE and return a vector of results.
// Propagation currently fixed at 1 min intervals for 24 hrs
pub fn propagate_tles(tle_list: Vec<TLE>) -> Result<Vec<PropagationResult>> {
    let mut results: Vec<PropagationResult> = Vec::new();

    for tle in tle_list {
        let elements = sgp4::Elements::from_tle(tle.name.clone(), tle.line1.as_bytes(), tle.line2.as_bytes())?;
        let constants = sgp4::Constants::from_elements(&elements)?;
        let mut predictions: Vec<Prediction> = Vec::new();

        for minutes in 0..1440 {
            let t = minutes as f64;
            predictions.push(constants.propagate(sgp4::MinutesSinceEpoch(t))?);
        }

        results.push(PropagationResult { 
            object_name: tle.name.unwrap_or_else(|| "Unnamed object".to_owned()), 
            predictions: predictions 
        });
    }

    Ok(results)
}


// TESTS
// TODO