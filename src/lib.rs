pub mod tle;
pub mod fetcher;
pub mod plotter;
pub mod propagator;
pub mod cli;

use anyhow::Result;

use cli::{CLI, Commands};

use crate::cli::QueryArgs;


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
    let tle_string = fetcher::query_celestrak(&args.query, &args.value)?;
    println!("{tle_string:?}");

    // TODO: call TLE parser
    // TODO: call TLE propagator
    // TODO: call TLE plotter

    return Ok(());
}