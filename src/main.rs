use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use crate::sprites::spawn_hoverable_sprite;

mod sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    // commands.spawn((Sprite {
    //     image: asset_server.load("ducky.png"),
    //     ..Default::default()
    // },
    // Pickable::default(),
    // ))
    // .observe(on_pointer_change::<PointerOver>)
    // .observe(on_pointer_change::<PointerOut>);
    spawn_hoverable_sprite(commands, asset_server.load("ducky.png"), None, None, None, None);
}

// fn on_pointer_change<E: EntityEvent>(event: On<E>) {
//     println!("Pointer event");
// }
