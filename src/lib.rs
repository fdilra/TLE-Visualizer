pub mod tle;
pub mod fetcher;
pub mod graphics;
pub mod propagator;
pub mod cli;

use anyhow::{Context, Result};

use crate::cli::{CLI, Commands, QueryArgs};
use crate::tle::{TLE, parse_tles};
use crate::propagator::propagate_tles;
use crate::graphics::plot_tles;


pub fn run(cli: CLI) -> Result<()> {
    // Handle output flag
    if let Some(out_path) = cli.output_path.as_deref() {
        println!("Value for output_path: {out_path}");
    }

    // Handle commands
    if let Some(command) = cli.command {
        match command {
            Commands::Plot(args) => execute_plot_command(&args)?
        }
    } else {
        println!("\nNo command provided!\n\nUse the '-h' flag for help");
    }

    Ok(())
}

fn execute_plot_command(args: &QueryArgs) -> Result<()> {
    // Hardcoded string for testing (to avoid getting ip banned from Celestrak for too many requests)
    let test_string = "ISS (ZARYA)\n1 25544U 98067A   25260.12361477  .00008550  00000-0  15572-3 0  9997\n2 25544  51.6329 211.3907 0004353 348.5756  11.5133 15.50345634529426".to_owned();
    let tle_string = fetcher::query_celestrak(&args.query, &args.value)?;
    // println!("\nFetched TLE string:\n {:?}", &tle_string);

    let tle_list: Vec<TLE> = parse_tles(&tle_string)?;
    // println!("\n{:?}", tle_list);

    let propagation_results = propagate_tles(tle_list)?;
    // println!("{:?}", propagation_results);
    
    plot_tles(&propagation_results)?;

    return Ok(());
}