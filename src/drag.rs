use bevy::{
    camera::Camera, ecs::{
        component::Component, query::With, system::{Query, Res, ResMut},
    }, input::{ButtonInput, mouse::MouseButton}, math::Vec3, sprite::Sprite, time::Time, transform::components::{GlobalTransform, Transform}, window::{PrimaryWindow, Window},
};

use crate::{audio::AudioFactory, physics::PhysicsState, sprite::{SpriteAudioSettings, SpriteVisualSettings}};

#[derive(Component, Default)]
pub struct DragState {
    pub dragging: bool,
    pub last_world_position: Vec3,
}

// looks for entities with a drag state component and if dragging is enabled then
// have it follow the cursor
pub fn update_dragged_sprites(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
    mut sprites: Query<(&mut DragState, &mut PhysicsState, &mut Transform)>,
) {
    for (mut drag_state, mut physics_state, mut transform) in &mut sprites {
        if drag_state.dragging {
            let Ok(window) = windows.single() else {
                return;
            };

            let Ok((camera, camera_transform)) = cameras.single() else {
                return;
            };

            let Some(cursor_position) = window.cursor_position() else {
                return;
            };

            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                transform.translation = world_pos.extend(transform.translation.z);
                physics_state.set_velocity(
                    (transform.translation - drag_state.last_world_position) / time.delta_secs(),
                );
                drag_state.last_world_position = transform.translation;
            }
        }
    }
}

pub fn stop_dragging_on_mouse_release(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut audio_factory: ResMut<AudioFactory>,
    mut draggables: Query<(&mut Sprite, &mut DragState, Option<&mut PhysicsState>, Option<&SpriteAudioSettings>, Option<&SpriteVisualSettings>)>,
) {
    if !mouse_buttons.just_released(MouseButton::Left) {
        return;
    }

    for (mut sprite, mut drag_state, physics_state, audio_settings, visual_settings) in &mut draggables {
        if drag_state.dragging {
            drag_state.dragging = false;

            if let Some(mut physics_state) = physics_state {
                physics_state.enable();
            }

            if let Some(audio_settings) = audio_settings {
                if let Some(release_sound) = audio_settings.release_sound.clone() {
                    audio_factory.play_sound(release_sound);
                }
            }

            if let Some(visual_settings) = visual_settings {
                sprite.image = visual_settings.default_img.clone();
            }
        }
    }
}
