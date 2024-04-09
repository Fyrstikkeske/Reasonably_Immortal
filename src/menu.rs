use bevy::prelude::*;

use super::GameState;

pub fn menu_plugin(app: &mut App) {
    app
    .init_state::<MenuState>()
}