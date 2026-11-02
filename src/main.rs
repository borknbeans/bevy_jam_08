use bevy::prelude::*;
use bevy::{asset::AssetMetaCheck, picking::hover::Hovered};

use crate::audio::{AudioBundle, AudioCategory};

mod audio;
mod drag;
mod physics;
mod sprite;
mod sprite_factory;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Jam 08".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                audio::process_audio_requests,
                drag::update_dragged_sprites,
                drag::stop_dragging_on_mouse_release,
                physics::update_physics_sprites,
                sprite::update_trash_can_visibility,
            ),
        )
        .insert_resource(audio::AudioFactory::default())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Frog leg factory
    let frog_bag = sprite::ConfigurableSprite::default()
        .with_visual_settings(sprite::SpriteVisualSettings {
            default_img: asset_server.load("art/frog-bag.png"),
            hover_img: Some(asset_server.load("art/frog-bag_hover.png")),
            ..default()
        })
        .with_audio_settings(sprite::SpriteAudioSettings {
            hover_enter_sound: Some(AudioBundle::new(
                asset_server.load("audio/bag-close.ogg"),
                AudioCategory::SOUND_EFFECT,
            )),
            hover_exit_sound: Some(AudioBundle::new(
                asset_server.load("audio/bag-close.ogg"),
                AudioCategory::SOUND_EFFECT,
            )),
            ..default()
        })
        .spawn_at(&mut commands, Vec3::new(0.0, -247.0, 0.0), false);
    commands
        .entity(frog_bag)
        .insert(sprite_factory::SpriteFactory {
            sprite: sprite::ConfigurableSprite::default()
                .with_visual_settings(sprite::SpriteVisualSettings {
                    default_img: asset_server.load("art/frog-leg.png"),
                    press_img: Some(asset_server.load("art/frog-leg_press.png")),
                    ..default()
                })
                .draggable()
                .apply_physics(),
        });

    let trash_can = sprite::ConfigurableSprite::default()
        .with_visual_settings(sprite::SpriteVisualSettings {
            default_img: asset_server.load("art/trash-can.png"),
            hover_img: Some(asset_server.load("art/trash-can_hover.png")),
            ..default()
        })
        .with_audio_settings(sprite::SpriteAudioSettings {
            hover_enter_sound: Some(AudioBundle::new(
                asset_server.load("audio/bag-close.ogg"),
                AudioCategory::SOUND_EFFECT,
            )),
            hover_exit_sound: Some(AudioBundle::new(
                asset_server.load("audio/bag-close.ogg"),
                AudioCategory::SOUND_EFFECT,
            )),
            ..default()
        })
        .visible(false)
        .spawn_at(&mut commands, Vec3::new(600.0, -310.0, 0.5), false);
    commands
        .entity(trash_can)
        .insert((sprite::TrashCan, Hovered::default()));
}

// TODO: a trash can to delete sprites are are dropped into it
// TODO: a way to tag a sprite as an "ingredient"
// TODO: A "cauldron" that allows you to place ingredients into it and "create" a potion
// TODO: the cauldron should read from available recipes and transform the inputs into an output
// sprite
// TODO: customers
