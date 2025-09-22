
# TLE Visualizer

A Rust CLI tool for fetching, propagating, and visualizing satellite orbital data from [CelesTrak](https://celestrak.org).


![alt text](https://github.com/fdilra/TLE-Visualizer/blob/main/3d_preview.png "3D visualization preview")

---
## Features

- Query [CelesTrak](https://celestrak.org/NORAD/elements/) for TLE (Two-Line Element) data
- Propagate orbital elements using SGP4
- Visualize trajectory around the Earth in 3D using the `kiss3d` crate

---
## Usage

### Example: Query the ISS TLE by Catalog Number

```bash

tle-plot plot CATNR 25544

```

This queries CelesTrak for the ISS (NORAD ID `25544`) and returns its latest TLE set.

### Options

-  `-h, --help`: Print help

### Commands:
  
-  `plot <QUERY> <VALUE>`: Fetch TLEs, propagate, and open 3D visualization window
 
---
## Development Status

At present, the application can:

* Parse CLI arguments
* Fetch TLE data from CelesTrak
* Validate queries
* Visualize 3D trajectories

Next steps:
* Improve Earth model alignment with axes
* Add option for custom propagation time
* Add a command for 2D ground track visualization 
* Improve testing
* Improve presentation (background stars, atmosphere, illumination)

---
## Testing

Run tests with:

```bash

cargo  test

```

Some tests will make network calls to CelesTrak to fetch live TLEs.

---
