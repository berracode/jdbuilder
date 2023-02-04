use clap::{
    Args,
    Subcommand
};

#[derive(Debug, clap::Parser)]
pub struct CreateCommand{

    #[command(subcommand)]
    pub command: CreateSubcommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum CreateSubcommand{
    /// crea las entities de todas las tablas
    Entities(CreateEntities),
    /// Crea los dtos de todas las tablas
    Dtos(CreateDtos)
}

#[derive(Args, Debug)]
pub struct CreateEntities{
    /// The name of the user
    pub name: String,

    /// The email address of the user
    pub email: String,

    /// id de conexion a bd
    #[arg(short, long)]
    pub database: u8,
    
}

#[derive(Args, Debug)]
pub struct CreateDtos{
    /// The name of the user
    pub name: String,

    /// The email address of the user
    pub email: String,
    
}
