use anyhow::Result;
use sgp4::{Prediction};

use crate::tle::TLE;

#[derive(Debug)]
pub struct PropagationResult {
    pub object_name: String,
    pub positions: Vec<[f64; 3]>
}

// Propagate each fetched TLE and return a vector of results.
// Propagation currently fixed at 1 min intervals for 4 hrs
pub fn propagate_tles(tle_list: Vec<TLE>, optional_time: Option<i32>) -> Result<Vec<PropagationResult>> {
    let mut results: Vec<PropagationResult> = Vec::new();

    for tle in tle_list {
        let elements = sgp4::Elements::from_tle(
            tle.name.clone(),
            tle.line1.as_bytes(),
            tle.line2.as_bytes(),
        )?;
        let constants = sgp4::Constants::from_elements(&elements)?;
        let mut predictions: Vec<Prediction> = Vec::new();
        let mut positions: Vec<[f64; 3]> = Vec::new();

        // Set propagation time as optional cli argument or as default (4 hours)
        let propagation_time = match optional_time {
            Some(t) => t * 60,
            None => 240
        };

        for minutes in 0..propagation_time {
            let t = minutes as f64;
            predictions.push(constants.propagate(sgp4::MinutesSinceEpoch(t))?);
        }

        for prediction in predictions {
            // Convert from km to Earth radii
            positions.push(prediction.position);
        }

        results.push(PropagationResult {
            object_name: tle.name.unwrap_or_else(|| "Unnamed object".to_owned()),
            positions,
        });
    }

    Ok(results)
}


// TESTS
// TODO