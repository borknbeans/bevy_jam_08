use bevy::{
    camera::Camera,
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, mouse::MouseButton},
    math::Vec3,
    time::Time,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::physics::PhysicsState;

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
                transform.translation = world_pos.extend(0.0);
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
    mut draggables: Query<(&mut DragState, Option<&mut PhysicsState>)>,
) {
    if !mouse_buttons.just_released(MouseButton::Left) {
        return;
    }

    for (mut drag_state, physics_state) in &mut draggables {
        if drag_state.dragging {
            drag_state.dragging = false;

            if let Some(mut physics_state) = physics_state {
                physics_state.enable();
            }
        }
    }
}
