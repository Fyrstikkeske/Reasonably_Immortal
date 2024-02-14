use bevy::prelude::*;

use crate::player;

#[derive(Component)]
pub struct Playerdata{
	pub playernumber: u16,
}



pub fn move_camera_to_player_system(
	player: Query<(&Transform, &player::Playerdata), Without<Camera>>,
	mut camera: Query<&mut Transform, With<Camera>>
){
	for mut cameratransform in camera.iter_mut(){
		for (playerposition, playerdata) in player.iter(){
			if playerdata.playernumber == 0{
				cameratransform.translation = playerposition.translation;
			}
		}		
	}
}
