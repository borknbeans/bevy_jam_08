use bevy::{ecs::{component::Component, system::Commands }, math::Vec3} ;

use crate::sprite::ConfigurableSprite;

#[derive(Component)]
pub struct SpriteFactory {
    pub sprite: ConfigurableSprite,
}

impl SpriteFactory {
    pub fn spawn(&self, commands: &mut Commands, factory_position: Vec3) {
        let spawn_position = factory_position + Vec3::Z;

        self.sprite.clone().spawn_at(
            commands, 
            spawn_position, 
            true // start dragging
        );
    }
}
