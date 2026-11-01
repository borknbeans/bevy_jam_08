use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, sprites::update_dragged_sprites)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    sprites::ConfigurableSprite::new(asset_server.load("test_button.png"))
        .with_hover_image(asset_server.load("test_button_hover.png"))
        .with_press_image(asset_server.load("test_button_pressed.png"))
        .draggable()
        .spawn(commands);
}
