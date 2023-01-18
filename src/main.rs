
mod command;

use command::RustflixArgs;
use clap::Parser;

fn main() {
   let args = RustflixArgs::parse();
}