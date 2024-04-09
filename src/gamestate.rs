use bevy::ecs::schedule::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

// im definetly not copying the bevy examples
// anyways, put values that dont change beetwen menues and such here. In case i forger