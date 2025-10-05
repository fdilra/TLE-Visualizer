use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CLI {

    /// Option to set a custom propagation time. Default is 4 hours.
    #[arg(short, value_name = "PROPAGATION_TIME_IN_HOURS", global = true)]
    pub time: Option<u32>, 

    /// CLI commands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 3D plotting command
    Plot3d(QueryArgs),
    /// Ground track plotting command
    Plot2d(QueryArgs),
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

// TODO
// TESTS
