use clap::{
    Args,
    Subcommand
};



#[derive(Args, Debug)]
pub struct VideoCommand{

    #[clap(subcommand)]
    pub command: VideoSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum VideoSubcommand{
    /// Create a video
    Create(CreateVideo),
    /// Update a video
    Update(UpdateVideo)
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