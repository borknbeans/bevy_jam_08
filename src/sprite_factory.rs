use bevy::ecs::{component::Component, system::Commands } ;

use crate::sprite::ConfigurableSprite;

#[derive(Component)]
pub struct SpriteFactory {
    pub sprite: ConfigurableSprite,
}

impl SpriteFactory {
    pub fn spawn(&self, mut commands: Commands) {
        self.sprite.clone().spawn(commands);
    }
}
