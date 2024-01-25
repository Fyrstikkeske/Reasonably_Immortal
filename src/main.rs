use bevy::prelude::*;


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
//        .add_systems(Update)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(10.0, 0.15, 10.0))),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });


    //OMGAWD its MRGOOF
    commands.spawn((
    	PbrBundle {
    		mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 2.0)))),
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
