use std::f64::consts::PI;

use anyhow::Result;

use crate::tle::TLE;

#[derive(Debug, Clone)]
pub struct PropagationResult {
    pub object_name: String,
    pub positions: Vec<[f64; 3]>
}

// Propagate each fetched TLE at 1 min intervals and return a vector of results
pub fn propagate_tles(tle_list: Vec<TLE>, optional_time: Option<u32>) -> Result<Vec<PropagationResult>> {
    let mut results: Vec<PropagationResult> = Vec::new();

    for tle in tle_list {
        let elements = sgp4::Elements::from_tle(
            tle.name.clone(),
            tle.line1.as_bytes(),
            tle.line2.as_bytes(),
        )?;
        let constants = sgp4::Constants::from_elements(&elements)?;
        let mut positions: Vec<[f64; 3]> = Vec::new();

        // Set propagation time as optional cli argument or as default (4 hours)
        let propagation_time = match optional_time {
            Some(t) => t * 60,
            None => 240
        };

        for minutes in 0..propagation_time {
            let t = minutes as f64;
            let prediction = constants.propagate(sgp4::MinutesSinceEpoch(t))?;

            // Calculate ERA (Earth Rotation Angle in rad)
            let jd = (elements.epoch() * 365.256) + (t / 1440.0); // Julian date in days
            let era = 2.0 * PI * (0.7790572732640 + 1.00273781191135448 * (jd - 2451545.0));
            let era = era % (2.0 * PI); // keep in [0, 2PI)

            // Coords in TEME
            let x_teme = prediction.position[0];
            let y_teme = prediction.position[1];
            let z_teme = prediction.position[2];

            // Rotation around z axis
            let x_ecef =  x_teme * era.cos() - y_teme * era.sin();
            let y_ecef =  x_teme * era.sin() + y_teme * era.cos();
            let z_ecef =  z_teme;

            positions.push([x_ecef, y_ecef, z_ecef]);
        }

        // println!("{:?}", positions);

        results.push(PropagationResult {
            object_name: tle.name.unwrap_or_else(|| "Unnamed object".to_owned()),
            positions,
        });
    }

    Ok(results)
}


// TESTS
// TODO