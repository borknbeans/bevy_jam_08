use bevy::{
    asset::Handle, audio::{AudioPlayer, AudioSource, PlaybackSettings}, ecs::{
        component::Component, observer::On, system::{Commands, Query, Res, ResMut},
    }, image::Image, picking::{
        Pickable,
        events::{PointerOut, PointerOver, PointerPress, PointerRelease},
    }, sprite::Sprite, utils::default,
};
use rand::RngExt;

use crate::{
    audio::AudioBundle, drag::DragState, physics::{self, PhysicsState},
};

pub struct ConfigurableSprite {
    pub default_image: Handle<Image>,
    pub hover_image: Option<Handle<Image>>,
    pub press_image: Option<Handle<Image>>,
    pub hover_sound_fx: Option<Handle<AudioSource>>,
    pub press_sound_fx: Option<Handle<AudioSource>>,
    pub bounce_sound_fx: Option<Handle<AudioSource>>,
    pub visual_settings: SpriteVisualSettings,
    pub audio_settings: SpriteAudioSettings,
    pub draggable: bool,
    pub apply_physics: bool,
}

#[derive(Clone, Component, Default)]
pub struct SpriteVisualSettings {
    default_img: Option<Handle<Image>>,
    hover_img: Option<Handle<Image>>,
    press_img: Option<Handle<Image>>,
}

#[derive(Clone, Component, Default)]
pub struct SpriteAudioSettings {
    pub hover_enter_sound: Option<AudioBundle>,
    pub hover_exit_sound: Option<AudioBundle>,
    pub press_sound: Option<AudioBundle>,
    pub release_sound: Option<AudioBundle>,
    pub bounce_sound: Option<AudioBundle>,
}

impl ConfigurableSprite {
    pub fn new(default_image: Handle<Image>) -> Self {
        Self {
            default_image,
            hover_image: None,
            press_image: None,
            hover_sound_fx: None,
            press_sound_fx: None,
            bounce_sound_fx: None,
            visual_settings: SpriteVisualSettings::default(),
            audio_settings: SpriteAudioSettings::default(),
            draggable: false,
            apply_physics: false,
        }
    }

    // set a new image when hovered
    pub fn with_hover_image(mut self, image: Handle<Image>) -> Self {
        self.hover_image = Some(image);
        self
    }

    // set a new image when pressed
    pub fn with_press_image(mut self, image: Handle<Image>) -> Self {
        self.press_image = Some(image);
        self
    }

    // set hover sound effect
    #[allow(dead_code)]
    pub fn with_hover_sound(mut self, sound_fx: Handle<AudioSource>) -> Self {
        self.hover_sound_fx = Some(sound_fx);
        self
    }

    // set press sound effect
    pub fn with_press_sound(mut self, sound_fx: Handle<AudioSource>) -> Self {
        self.press_sound_fx = Some(sound_fx);
        self
    }

    // set bounce sound effect
    pub fn with_bounce_sound(mut self, sound_fx: Handle<AudioSource>) -> Self {
        self.bounce_sound_fx = Some(sound_fx);
        self
    }

    // enable dragging with the cursor
    pub fn draggable(mut self) -> Self {
        self.draggable = true;
        self
    }

    pub fn apply_physics(mut self) -> Self {
        self.apply_physics = true;
        self
    }

    pub fn spawn(self, mut commands: Commands) {
        let mut entity = commands.spawn((
            Sprite {
                image: self.default_image.clone(),
                ..default()
            },
            Pickable::default(),
        ));

        // optionally configure picking setup for hovering enter/exit
        if let Some(hover_image) = self.hover_image {
            let default_image = self.default_image.clone();

            entity.observe(
                move |event: On<PointerOver>,
                      mut sprites: Query<&mut Sprite>,
                      mut drag_states: Query<&mut DragState>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        if let Ok(drag_state) = drag_states.get_mut(event.entity)
                            && drag_state.dragging
                        {
                            return;
                        }
                        sprite.image = hover_image.clone();
                    }
                },
            );
            entity.observe(
                move |event: On<PointerOut>,
                      mut sprites: Query<&mut Sprite>,
                      mut drag_states: Query<&mut DragState>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        if let Ok(drag_state) = drag_states.get_mut(event.entity)
                            && drag_state.dragging
                        {
                            return;
                        }
                        sprite.image = default_image.clone();
                    }
                },
            );
        }

        // optionally configure audio playback for hovering
        if let Some(hover_sound_fx) = self.hover_sound_fx {
            entity.observe(move |_: On<PointerOver>, mut audio_factory: ResMut<crate::audio::AudioFactory>| {
                audio_factory.play_sound_effect(hover_sound_fx.clone(), crate::audio::AudioCategory::SOUND_EFFECT);
            });
        }

        // optionally configure picking setup for pressing/releasing
        if let Some(press_image) = self.press_image {
            let default_image = self.default_image.clone();

            entity.observe(
                move |event: On<PointerPress>, mut sprites: Query<&mut Sprite>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        sprite.image = press_image.clone();
                    }
                },
            );
            entity.observe(
                move |event: On<PointerRelease>, mut sprites: Query<&mut Sprite>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        sprite.image = default_image.clone();
                    }
                },
            );
        }

        // optionally configure audio playback for pressing
        if let Some(press_sound_fx) = self.press_sound_fx {
            entity.observe(move |_: On<PointerPress>, mut audio_factory: ResMut<crate::audio::AudioFactory>| {
                audio_factory.play_sound_effect(press_sound_fx.clone(), crate::audio::AudioCategory::SOUND_EFFECT);
            });
        }

        // optionally configure the sprite being draggable
        if self.draggable {
            entity.insert(DragState::default());

            entity.observe(
                |event: On<PointerPress>,
                 mut drag_states: Query<&mut DragState>,
                 mut physics_states: Query<&mut physics::PhysicsState>| {
                    if let Ok(mut drag_state) = drag_states.get_mut(event.entity) {
                        drag_state.dragging = true;
                    }

                    if let Ok(mut physics_state) = physics_states.get_mut(event.entity) {
                        physics_state.disable();
                        physics_state.clear_velocity();
                    }
                },
            );
        }

        // optionally enable physics on sprite (gravity and velocity)
        if self.apply_physics {
            let mut physics_state = PhysicsState::new();
            if let Some(bounce_sound_fx) = self.bounce_sound_fx {
                physics_state.set_bounce_fx(bounce_sound_fx);
            }
            entity.insert(physics_state);
        }
    }
}
