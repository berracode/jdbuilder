
mod command;

use command::{
   RustflixArgs, 
   EntityType::User, 
   EntityType::Video, 
   user_command::{
      UserCommand, 
      CreateUser
   }, 
   video_command::{
      VideoCommand,
      CreateVideo, UpdateVideo 
   }
};
use clap::Parser;

use crate::command::video_command::VideoSubcommand;

fn main() {

   let args = RustflixArgs::parse();
   let entity_type = args.entity_type;

   match entity_type {
      User(user) => handle_user_command(user),
      Video(video) => handle_video_command(video)
   }
}

fn handle_user_command(user: UserCommand){
   println!("Doing something important...");
}

fn handle_video_command(video: VideoCommand){
   println!("Doing something important... {:?}",video);

   match video.command {
      VideoSubcommand::Create(createVideo) => create_video(createVideo),
      VideoSubcommand::Update(updateVideo) => update_video(updateVideo),
   }

}

fn create_video(video: CreateVideo){
   println!("Creating video... ");
   //TODO write in file the video info
}

fn update_video(video: UpdateVideo){
   println!("Updating video... ");
   //TODO update in file the video info
   
}