use bevy::{asset::AssetServer, ecs::{resource::Resource, system::{Res, ResMut}}, platform::collections::HashMap, utils::default};

use crate::sprite::{ConfigurableSprite, SpriteVisualSettings};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum ItemId {
    FrogLeg,
}

#[derive(Resource, Default)]
pub struct ItemRegistry {
    items: HashMap<ItemId, ConfigurableSprite>,
}

impl ItemRegistry {
    pub fn register(&mut self, id: ItemId, sprite: ConfigurableSprite) {
        self.items.insert(id, sprite);
    }

    pub fn get(&self, id: ItemId) -> Option<&ConfigurableSprite> {
        self.items.get(&id)
    }
}

pub fn register_items(
    mut registry: ResMut<ItemRegistry>,
    asset_server: Res<AssetServer>,
) {
    registry.register(
        ItemId::FrogLeg,
        ConfigurableSprite::default()
            .with_id(ItemId::FrogLeg)
            .with_visual_settings(SpriteVisualSettings {
                default_img: asset_server.load("art/frog-leg.png"),
                press_img: Some(asset_server.load("art/frog-leg_press.png")),
                ..default()
            })
            .draggable()
            .apply_physics(),
    );
}
