use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::player::Playerdata;


#[derive(Component)]
struct Camera;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 0.15, 10.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(10.0, 0.15, 10.0))),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
    },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(
    (Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
	Camera
    ));


    //OMGAWD its MRGOOF
    commands.spawn((
    	Playerdata{playernumber: 0},
		RigidBody::Dynamic,
		Collider::cuboid(1.0, 1.0, 0.002),
		LockedAxes::from_bits(0b000_111),
    	PbrBundle {
    		mesh: meshes.add(Mesh::from(Rectangle::new(1.0, 1.0))),
    		transform: Transform::from_xyz(0.0, 0.0, 0.0),
    		material: materials.add(
    			StandardMaterial {
    				base_color_texture: Some(asset_server.load("Mr_goof.png")),
    				perceptual_roughness: 1.0,
					alpha_mode: AlphaMode::Mask(0.5),
					cull_mode: None,
					..default()
    			}
    		),
    		..default()
    	},
    	));
}
