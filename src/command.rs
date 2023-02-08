use clap::{
    Subcommand
};

pub mod user_command;
pub mod video_command;
pub mod create_command;


use user_command::UserCommand;
use video_command::VideoCommand;
use create_command::CreateCommand;


#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RustflixArgs {
    #[command(subcommand)]
    pub entity_type: EntityType,

}


#[derive(Subcommand, Debug)]
pub enum EntityType{
    /// Create, update, delete or show users
    User(UserCommand),

    /// Create, update, delete or show videos
    Video(VideoCommand),

    /// Create
    Create(CreateCommand)
}
