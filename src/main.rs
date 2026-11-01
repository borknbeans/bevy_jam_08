use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

mod audio;
mod drag;
mod physics;
mod sprite;
mod sprite_factory;

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
                audio::process_audio_requests,
                drag::update_dragged_sprites,
                drag::stop_dragging_on_mouse_release,
                physics::update_physics_sprites,
            ),
        )
        .insert_resource(audio::AudioFactory::default())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
   
    // Frog leg factory
    let frog_leg_factory = sprite_factory::SpriteFactory {
        sprite: sprite::ConfigurableSprite::default()
            .with_visual_settings(sprite::SpriteVisualSettings {
                default_img: asset_server.load("art/frog-leg.png"),
                ..default()
            })
            .draggable()
            .apply_physics(),
    };
    sprite::ConfigurableSprite::default()
        .with_visual_settings(sprite::SpriteVisualSettings {
            default_img: asset_server.load("art/frog-bag.png"),
            hover_img: Some(asset_server.load("art/frog-bag_hover.png")),
            ..default()
        });
    
    // sprite::ConfigurableSprite::new(asset_server.load("art/potion.png"))
    //     .with_hover_image(asset_server.load("art/potion_hover.png"))
    //     .with_press_image(asset_server.load("art/potion_pressed.png"))
    //     .with_bounce_sound(asset_server.load("audio/glass-clink.ogg"))
    //     .with_press_sound(asset_server.load("audio/cork-pop.ogg"))
    //     .draggable()
    //     .apply_physics()
    //     .spawn(commands);
}

// TODO: ConfigurableSprite should optionally support animations! For example, hovering or pressing
// should play an animation

// TODO: a ConfigurableSpriteFactory that can spawn a new ConfigurableSprite when clicked on
// TODO: a trash can to delete sprites are are dropped into it
// TODO: a way to tag a sprite as an "ingredient"
// TODO: A "cauldron" that allows you to place ingredients into it and "create" a potion
// TODO: the cauldron should read from available recipes and transform the inputs into an output
// sprite
// TODO: customers
