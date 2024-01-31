use bevy::prelude::*;

mod setup;




fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, crate::setup::setup)
//        .add_systems(Update)
        .run();
}

