use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod setup;
mod player;



fn main(){
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, crate::setup::setup)
        .add_systems(Update, player::move_camera_to_player_system)
        .run();
}

