pub mod tle;
pub mod fetcher;
pub mod graphics;
pub mod graphics2d;
pub mod propagator;
pub mod cli;

use anyhow::{Result};

use crate::cli::{CLI, Commands, QueryArgs};
use crate::tle::{TLE, parse_tles};
use crate::propagator::propagate_tles;
use crate::graphics::plot_3d;
use crate::graphics2d::plot_2d;


pub fn run(cli: CLI) -> Result<()> {
    // Handle commands
    if let Some(command) = cli.command {
        match command {
            Commands::Plot3d(args) => execute_plot3d(&args, cli.time)?,
            Commands::Plot2d(args) => execute_plot2d(&args, cli.time)?
        }
    } else {
        println!("\nNo command provided!\n\nUse the '-h' flag for help");
    }

    Ok(())
}

fn execute_plot3d(args: &QueryArgs, time: Option<i32>) -> Result<()> {
    // Hardcoded string for testing (to avoid getting ip banned from Celestrak for too many requests)
    let tle_string = "ISS (ZARYA)\n1 25544U 98067A   25260.12361477  .00008550  00000-0  15572-3 0  9997\n2 25544  51.6329 211.3907 0004353 348.5756  11.5133 15.50345634529426".to_owned();
    let tle_string = fetcher::query_celestrak(&args.query, &args.value)?;
    // println!("\nFetched TLE string:\n {:?}", &tle_string);

    let tle_list: Vec<TLE> = parse_tles(&tle_string)?;
    // println!("\n{:?}", tle_list);

    let propagation_results = propagate_tles(tle_list, time)?;
    // println!("{:?}", propagation_results);
    
    plot_3d(&propagation_results)?;

    return Ok(());
}

fn execute_plot2d(args: &QueryArgs, time: Option<i32>) -> Result<()> {
    // Hardcoded string for testing (to avoid getting ip banned from Celestrak for too many requests)
    let tle_string = "ISS (ZARYA)\n1 25544U 98067A   25260.12361477  .00008550  00000-0  15572-3 0  9997\n2 25544  51.6329 211.3907 0004353 348.5756  11.5133 15.50345634529426".to_owned();
    let tle_string = fetcher::query_celestrak(&args.query, &args.value)?;
    // println!("\nFetched TLE string:\n {:?}", &tle_string);

    let tle_list: Vec<TLE> = parse_tles(&tle_string)?;
    // println!("\n{:?}", tle_list);

    let propagation_results = propagate_tles(tle_list, time)?;
    // println!("{:?}", propagation_results);
    
    plot_2d(&propagation_results)?;

    return Ok(());
}