use bevy::ecs::component::Component;

use crate::registry::ItemId;


#[derive(Component, Default)]
pub struct Cauldron {
    pub ingredients: Vec<ItemId>,
}
