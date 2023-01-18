use clap::{
    Args,
    Subcommand
};

#[derive(Args, Debug)]
pub struct UserCommand{

    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum UserSubcommand{
    /// Create an user
    Create(CreateUser),
    /// Update an user
    Update(UpdateUser)
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
