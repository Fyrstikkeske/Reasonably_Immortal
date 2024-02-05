use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod setup;




fn main(){
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, crate::setup::setup)
//        .add_systems(Update)
        .run();
}

