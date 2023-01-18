use clap::{
    Args,
    Subcommand
};

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
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
}

#[derive(Args, Debug)]
pub struct UserCommand{

    #[clap(subcommand)]
    pub command: UserSubcommand,
}


#[derive(Args, Debug)]
pub struct VideoCommand{

    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum UserSubcommand{
    /// Create an user
    Create(CreateUser),
    /// Update an user
    Update(UpdateUser)
}

#[derive(clap::Subcommand, Debug)]
pub enum VideoSubcommand{
    /// Create a video
    Create(CreateVideo),
    /// Update a video
    Update(UpdateVideo)
}

#[derive(Args, Debug)]
pub struct CreateUser{
    /// The name of the user
    pub name: String,

    /// The email address of the user
    pub email: String,
    
}

#[derive(Args, Debug)]
pub struct UpdateUser{
    /// The name of the user
    pub name: String,

    /// The email address of the user
    pub email: String,
    
}

#[derive(Args, Debug)]
pub struct CreateVideo{
    /// The video of the user
    pub format: String,

    /// The email address of the user
    pub duration: i32,
    
}

#[derive(Args, Debug)]
pub struct UpdateVideo{
    /// The name of the user
    pub format: String,

    /// The email address of the user
    pub duration: i32,
    
}