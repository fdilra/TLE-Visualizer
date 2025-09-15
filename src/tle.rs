use anyhow::{bail, Result};

#[derive(Debug)]
pub struct TLE {
    pub name: Option<String>,
    pub line1: String,
    pub line2: String
}

pub fn parse_tles(tle_str: &str) -> Result<Vec<TLE>> {
    let mut tle_list: Vec<TLE> = Vec::new();
    let mut lines = tle_str.lines().map(str::trim).filter(|l| !l.is_empty());

    while let Some(first) = lines.next() {
        let (name, l1);

        if first.starts_with('1') {
            // No name line, this is line 1
            name = None;
            l1 = first;
        } else if first.starts_with('2') {
            bail!("TLE group started with line 2, missing line 1");
        } else {
            // Name line is present
            name = Some(first.to_string());

            l1 = match lines.next() {
                Some(l) if l.starts_with('1') => l,
                _ => bail!("Missing or invalid line 1 after name"),
            };
        }

        let l2 = match lines.next() {
            Some(l) if l.starts_with('2') => l,
            _ => bail!("Missing or invalid line 2"),
        };

        tle_list.push(TLE {
            name,
            line1: l1.to_string(),
            line2: l2.to_string(),
        });
    }

    return Ok(tle_list);
}


// TESTS
// TODO