
mod command;

use command::{
   RustflixArgs, 
   EntityType::User, 
   EntityType::Video, 
   EntityType::Create,
   user_command::{
      UserCommand, 
      CreateUser
   }, 
   video_command::{
      VideoCommand,
      CreateVideo, UpdateVideo 
   },
   create_command::{
      CreateCommand,
      CreateEntities, CreateDtos
   }
};
use clap::Parser;
use command::mysql_adapter;

use crate::command::{video_command::VideoSubcommand, mysql_adapter::describe_table};

fn main() {

   let args = RustflixArgs::parse();
   let entity_type = args.entity_type;

   match entity_type {
      User(user) => handle_user_command(user),
      Video(video) => handle_video_command(video),
      Create(_) => todo!(),
   }
}

fn handle_user_command(user: UserCommand){
   println!("Doing something important...");
}

fn handle_video_command(video: VideoCommand){
   println!("Doing something important... {:?}",video);

   match video.command {
      VideoSubcommand::Create(video) => create_video(video),
      VideoSubcommand::Update(video) => update_video(video),
   }

}

fn create_video(video: CreateVideo){
   println!("Creating video... ");
   //TODO write in file the video info
   describe_table();
}

fn update_video(video: UpdateVideo){
   println!("Updating video... ");
   //TODO update in file the video info
   
}