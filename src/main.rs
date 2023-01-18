
mod args;

use args::RustflixArgs;
use clap::Parser;

fn main() {
   let args = RustflixArgs::parse();
}