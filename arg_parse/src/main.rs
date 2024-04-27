mod args;


use args::MainArgs;
use clap::Parser;
fn main() {
    let args:MainArgs = MainArgs::parse();
    // printing the parsed arguments
    println!("{:?}", args);
}
