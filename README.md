# TLE Plot

A Rust CLI tool for fetching, propagating, and plotting satellite orbital data from [CelesTrak](https://celestrak.org).
Currently in early development.

---


## Features (Work in Progress)

- Query [CelesTrak](https://celestrak.org/NORAD/elements/) for TLE (Two-Line Element) data
- Parse TLEs into structured data
- Propagate orbital elements
- Plot orbits and export results
- Support multiple output formats

  
---

## Usage

### Example: Query the ISS (ZARYA) TLE by Catalog Number

```bash

tle-plot  plot  CATNR  25544

```

This queries CelesTrak for the ISS (NORAD ID `25544`) and returns its latest TLE set.

### Options

-  `-o, --output-path <PATH>`: Specify directory to export plots (not yet implemented)
-  `-e, --extension`: Specify the file extension for the exported plot (requires "-o") (not yet implemented)

### Commands:

-  `plot <QUERY> <VALUE>`: Fetch TLEs, propagate, and plot (currently only fetches TLEs)

  
---

## Development Status

This project is in **early development**.

At present, it can:
* Parse CLI arguments
* Fetch TLE data from CelesTrak
* Validate queries

Next steps:
* Implement TLE parsing
* Add orbit propagation
* Add plotting and exporting

---
 

## Testing

Run tests with:

  

```bash
cargo  test
```

Some tests will make network calls to CelesTrak to fetch live TLEs.

---
