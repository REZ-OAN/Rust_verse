use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
// defining the structure
pub struct MainArgs{
    /// The first argument!!
    pub first_arg : String,
    /// The first argument!!
    pub second_arg : String,
    /// The first argument!!
    pub third_arg : String,
}