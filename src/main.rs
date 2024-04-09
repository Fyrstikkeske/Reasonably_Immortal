use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;


mod setup;
mod player;
mod gamestate;

fn main(){
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
		.init_state::<gamestate::GameState>()
        .add_systems(Startup, setup::setup)
		//things after this is stuff ill try and move now
        .add_systems(PostUpdate, player::move_camera_to_player_system.after(PhysicsSet::Sync).before(bevy::transform::TransformSystem::TransformPropagate))
        .add_systems(Update, (player::moveplayer, player::camerarot))
        .run();
}
