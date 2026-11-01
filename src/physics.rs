use bevy::{
    asset::{Assets, Handle}, audio::{AudioPlayer, AudioSource, PlaybackSettings}, camera::Camera, ecs::{
        component::Component, system::{Commands, Query, Res, ResMut},
    }, image::Image, math::{Vec2, Vec3, ops::abs}, sprite::Sprite, time::Time, transform::components::{GlobalTransform, Transform},
};
use rand::RngExt;

use crate::{audio::AudioFactory, sprite::SpriteAudioSettings};

const BOUNCE_AUDIO_THRESHOLD: f32 = 8.0;

#[derive(Component)]
pub struct PhysicsState {
    enabled: bool,
    gravity: f32,
    velocity: Vec2,
    bounce_factor: f32,
    roughness_factor: f32,
    bounce_sound_fx: Option<Handle<AudioSource>>,
}

impl PhysicsState {
    pub fn new() -> Self {
        Self {
            enabled: true,
            gravity: 2000.0,
            velocity: Vec2::new(0.0, 0.0),
            bounce_factor: 0.15,
            roughness_factor: 0.35,
            bounce_sound_fx: None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn clear_velocity(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
    }

    pub fn set_velocity(&mut self, v: Vec3) {
        self.velocity.x = v.x;
        self.velocity.y = v.y;
    }

    pub fn set_bounce_fx(&mut self, sound_fx: Handle<AudioSource>) {
        self.bounce_sound_fx = Some(sound_fx);
    }
}

pub fn update_physics_sprites(
    time: Res<Time>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    images: Res<Assets<Image>>,
    mut audio_factory: ResMut<AudioFactory>,
    mut sprites: Query<(&Sprite, &mut PhysicsState, &mut Transform, Option<&SpriteAudioSettings>)>,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    if let Some(viewport_size) = camera.logical_viewport_size() {
        let min_x = 0.0;
        let min_y = 0.0;
        let max_x = viewport_size.x;
        let max_y = viewport_size.y;

        let Ok(tl) = camera.viewport_to_world_2d(camera_transform, Vec2::new(min_x, min_y)) else {
            return;
        };
        let Ok(br) = camera.viewport_to_world_2d(camera_transform, Vec2::new(max_x, max_y)) else {
            return;
        };

        for (sprite, mut physics_state, mut transform, audio_settings) in &mut sprites {
            if physics_state.enabled {
                let sprite_size = sprite
                    .custom_size
                    .or_else(|| images.get(&sprite.image).map(|image| image.size_f32()));

                let Some(size) = sprite_size else {
                    continue; // image not loaded in yet
                };

                let half_size = size * transform.scale.truncate().abs() * 0.5;

                let left = tl.x + half_size.x;
                let right = br.x - half_size.x;
                let top = tl.y - half_size.y;
                let bottom = br.y + half_size.y;

                physics_state.velocity.y -= physics_state.gravity * time.delta_secs();

                transform.translation.x += physics_state.velocity.x * time.delta_secs();
                transform.translation.y += physics_state.velocity.y * time.delta_secs();

                let mut bounced: bool = false;
                if transform.translation.x < left {
                    transform.translation.x = left;
                    if physics_state.velocity.x < 0.0 {
                        physics_state.velocity.x =
                            -(physics_state.velocity.x * physics_state.bounce_factor);
                        if physics_state.velocity.x > BOUNCE_AUDIO_THRESHOLD {
                            bounced = true;
                        }
                    }
                }

                if transform.translation.x > right {
                    transform.translation.x = right;
                    if physics_state.velocity.x > 0.0 {
                        physics_state.velocity.x =
                            -(physics_state.velocity.x * physics_state.bounce_factor);
                        if physics_state.velocity.x < -BOUNCE_AUDIO_THRESHOLD {
                            bounced = true;
                        }
                    }
                }

                if transform.translation.y < bottom {
                    transform.translation.y = bottom;
                    if physics_state.velocity.y < 0.0 {
                        physics_state.velocity.y =
                            -(physics_state.velocity.y * physics_state.bounce_factor);
                        if physics_state.velocity.y > BOUNCE_AUDIO_THRESHOLD {
                            bounced = true;
                        }
                    }

                    // apply friction
                    if physics_state.velocity.x != 0.0 {
                        let vx = physics_state.velocity.x * physics_state.roughness_factor;
                        if abs(vx) <= 1.0 {
                            // threshold
                            physics_state.velocity.x = 0.0;
                        } else {
                            physics_state.velocity.x = vx;
                        }
                    }
                }

                if transform.translation.y > top {
                    transform.translation.y = top;
                    if physics_state.velocity.y > 0.0 {
                        physics_state.velocity.y =
                            -(physics_state.velocity.y * physics_state.bounce_factor);
                        if physics_state.velocity.y < -BOUNCE_AUDIO_THRESHOLD {
                            bounced = true;
                        }
                    }
                }

                if bounced {
                    if let Some(audio_settings) = audio_settings {
                        if let Some(bounce_sound) = audio_settings.bounce_sound.clone() {
                            audio_factory.play_sound(bounce_sound);
                        }
                    }
                }
            }
        }
    }
}
