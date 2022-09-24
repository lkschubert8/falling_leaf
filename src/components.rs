use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerLeaf {
    pub gusts_left: u32,
}

#[derive(Component)]
pub struct Mug {
    pub leafs_in_mug: u32,
}
