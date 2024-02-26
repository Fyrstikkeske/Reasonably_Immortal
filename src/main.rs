use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy::transform::TransformSystem;

mod setup;
mod player;



fn main(){
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, crate::setup::setup)
        .add_systems(PostUpdate, player::move_camera_to_player_system.after(PhysicsSet::Sync).before(TransformSystem::TransformPropagate))
        .add_systems(Update, moveplayer)
        .run();
}


fn moveplayer(
	mut query: Query<&mut LinearVelocity, With<player::Playerdata>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
){
	for mut linear_velocity in query.iter_mut() {
			
			let xvel = match(
				keyboard_input.pressed(KeyCode::KeyA),
				keyboard_input.pressed(KeyCode::KeyD) 
			){
				(true, false) => -1.0,
				(false, true) => 1.0,
				_ => 0.0,
			};
			
			let zvel = match(
				keyboard_input.pressed(KeyCode::KeyW),
				keyboard_input.pressed(KeyCode::KeyS) 
			){
				(true, false) => -1.0,
				(false, true) => 1.0,
				_ => 0.0,
			};
			
	        linear_velocity.x += xvel;
			linear_velocity.z += zvel;

	        
	}
}
