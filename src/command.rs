use clap::{
    Args,
    Subcommand
};

pub mod user_command;
pub mod video_command;

use user_command::UserCommand;
use video_command::VideoCommand;


#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,

}


#[derive(Subcommand, Debug)]
pub enum EntityType{
    /// Create, update, delete or show users
    User(UserCommand),

    /// Create, update, delete or show videos
    Video(VideoCommand),
}
