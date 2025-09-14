use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// Option to export plot to specified directory
    #[arg(short, long, value_name = "PATH")]
    pub output_path: Option<String>,

    // TODO: add -e option for specifying exported plot extension (requires -o) and decide default extension

    /// CLI commands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Main command, query celestrak for TLEs, propagate the response, and plot it
    Plot(QueryArgs),
}

#[derive(Args, Debug)]
pub struct QueryArgs {
    /// Allowed query types:
    /// 
    /// - CATNR: Catalog Number (1 to 9 digits). Allows return of data for a single catalog number.
    ///
    /// - INTDES: International Designator (yyyy-nnn). Allows return of data for all objects associated with a particular launch.
    ///
    /// - GROUP: Groups of satellites provided on the CelesTrak Current Data page.
    ///
    /// - NAME: Satellite Name. Allows searching for satellites by parts of their name.
    ///
    /// - SPECIAL: Special data sets for the GEO Protected Zone (GPZ) or GPZ Plus.
    ///
    pub query: String,

    /// Object value (e.g. 25544)
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plot_command() {
        let cli = CLI::parse_from([
            "testbin",
            "--output-path", "outdir",
            "plot",
            "CATNR",
            "25544"
        ]);

        assert_eq!(cli.output_path, Some("outdir".to_string()));

        match cli.command.unwrap() {
            Commands::Plot(args) => {
                assert_eq!(args.query, "CATNR");
                assert_eq!(args.value, "25544");
            }
        }
    }

    // TODO
}
