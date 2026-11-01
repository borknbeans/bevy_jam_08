use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod physics;
mod sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                sprites::update_dragged_sprites,
                physics::update_physics_sprites,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    sprites::ConfigurableSprite::new(asset_server.load("art/potion.png"))
        .with_hover_image(asset_server.load("art/potion_hover.png"))
        .with_press_image(asset_server.load("art/potion_pressed.png"))
        .with_bounce_sound(asset_server.load("audio/glass-clink.ogg"))
        .with_press_sound(asset_server.load("audio/cork-pop.ogg"))
        .draggable()
        .apply_physics()
        .spawn(commands);
}
