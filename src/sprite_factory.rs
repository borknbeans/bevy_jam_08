use bevy::{ecs::{component::Component, observer::On, system::{Commands, Query, ResMut}}, picking::{Pickable, events::{PointerOut, PointerOver, PointerPress}}, sprite::Sprite, utils::default};

use crate::{audio::AudioFactory, drag::DragState, physics::PhysicsState, sprite::ConfigurableSprite};

#[derive(Component)]
pub struct SpriteFactory {
    sprite: ConfigurableSprite,
}

impl SpriteFactory {
    pub fn spawn(&self, mut commands: Commands) {
        let mut entity = commands.spawn((
            Sprite {
                image: self.sprite.default_image.clone(),
                ..default()
            },
            self.sprite.audio_settings.clone(),
            self.sprite.visual_settings.clone(),
        ));

        if self.sprite.draggable {
            entity.insert(Pickable::default());
            entity.insert(DragState::default());

            // On click, set update drag state and (if applicable) disable physics
            entity.observe(
                |event: On<PointerPress>,
                mut drag_states: Query<&mut DragState>,
                mut physics_states: Query<&mut PhysicsState>| {
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

        // AUDIO
        if let Some(hover_enter_sound) = self.sprite.audio_settings.hover_enter_sound.clone() {
            entity.observe(
                move |_: On<PointerOver>, mut audio_factory: ResMut<AudioFactory>| {
                    audio_factory.play_sound(hover_enter_sound.clone());
                }
            );
        }

        if let Some(hover_exit_sound) = self.sprite.audio_settings.hover_exit_sound.clone() {
            entity.observe(
                move |_: On<PointerOut>, mut audio_factory: ResMut<AudioFactory>| {
                    audio_factory.play_sound(hover_exit_sound.clone());
                }
            );
        }

        if let Some(press_sound) = self.sprite.audio_settings.press_sound.clone() {
            entity.observe(
                move |_: On<PointerPress>, mut audio_factory: ResMut<AudioFactory>| {
                    audio_factory.play_sound(press_sound.clone());
                }
            );
        }

        // if let Some(release_sound) = self.sprite.audio_settings.release_sound.clone() {
        //     entity.observe(
        //         move |_: On<PointerOver>, mut audio_factory: ResMut<AudioFactory>| {
        //             audio_factory.play_sound(release_sound.clone());
        //         }
        //     );
        // }
        //
        // if let Some(bounce_sound) = self.sprite.audio_settings.bounce_sound.clone() {
        //     entity.observe(
        //         move |_: On<PointerOver>, mut audio_factory: ResMut<AudioFactory>| {
        //             audio_factory.play_sound(bounce_sound.clone());
        //         }
        //     );
        // }
    }
}
