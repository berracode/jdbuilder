
mod command;

use command::{RustflixArgs, EntityType, EntityType::User, EntityType::Video};
use clap::Parser;

fn main() {
   let args = RustflixArgs::parse();
   let entity_type = args.entity_type;

   match entity_type {
      User(user) => do_something(),
      Video(video) => do_something()
   }
}

fn do_something(){
   println!("Doing something important...");
}