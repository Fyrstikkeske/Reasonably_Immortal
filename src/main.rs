use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod setup;
mod player;



fn main(){
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, crate::setup::setup)
        .add_systems(Update, move_camera_to_player_system)
        .run();
}


fn move_camera_to_player_system(
	player: Query<(&Transform, &player::Playerdata)>,
	mut camera: Query<&mut Transform, With<Camera>>
){
	for mut cameratransform in camera.iter_mut(){
		for (playerposition, playerdata) in player.iter(){
			if playerdata.playernumber == 1{
				cameratransform.translation = playerposition.translation;
			}
		}		
	}
}
