use bevy::{input::mouse, prelude::*};
use bevy_xpbd_3d::prelude::*;
use bevy::transform::TransformSystem;

mod setup;
mod player;



fn main(){
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, crate::setup::setup)
        .add_systems(PostUpdate, player::move_camera_to_player_system.after(PhysicsSet::Sync).before(TransformSystem::TransformPropagate))
        .add_systems(Update, (moveplayer, camerarot))
        .run();
}


fn camerarot(
	mut query: Query<&mut Transform, With<Camera>>,
	mut motion: EventReader<mouse::MouseMotion>,
){
	for mut transform in query.iter_mut(){
		let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
		
		for ev in motion.read(){
			yaw += ev.delta.x.to_radians();
			pitch += ev.delta.y.to_radians();
		}
		pitch = pitch.clamp(-160.0, 160.0);

		transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
	}
}



fn moveplayer(
	mut query: Query<&mut LinearVelocity, With<player::Playerdata>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	camera: Query<&mut Transform, With<Camera>>,
){
	for mut linear_velocity in query.iter_mut() {

		let ca = camera.single();
		
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
		
		let (yaw,_,_) = ca.rotation.to_euler(EulerRot::YXZ);
	    linear_velocity.x += zvel*yaw.sin();
		linear_velocity.z += zvel*yaw.cos();
		
		linear_velocity.x += xvel*yaw.cos();
		linear_velocity.z -= xvel*yaw.sin();
		// you wont believe how long it took me to do this, especially whens 1 minute for each change
	        
	}
}
