use clap::Parser;

use tle_plot::cli::CLI;


fn main() {
    println!("{:?}", std::env::args().collect::<Vec<_>>());
    let cli = CLI::parse();
    
    match tle_plot::run(cli) {
        Err(e) => println!("Execution failed with error: {e}"),
        _ => ()
    }
}
