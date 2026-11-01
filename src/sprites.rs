use bevy::{
    asset::Handle,
    camera::Camera,
    ecs::{
        component::Component,
        observer::On,
        query::With,
        system::{Commands, Query},
    },
    image::Image,
    picking::{
        Pickable,
        events::{PointerOut, PointerOver, PointerPress, PointerRelease},
    },
    sprite::Sprite,
    transform::components::{GlobalTransform, Transform},
    utils::default,
    window::{PrimaryWindow, Window},
};

use crate::physics;

pub struct ConfigurableSprite {
    default_image: Handle<Image>,
    hover_image: Option<Handle<Image>>,
    press_image: Option<Handle<Image>>,
    draggable: bool,
    apply_physics: bool,
}

impl ConfigurableSprite {
    pub fn new(default_image: Handle<Image>) -> Self {
        Self {
            default_image,
            hover_image: None,
            press_image: None,
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
                move |event: On<PointerOver>, mut sprites: Query<&mut Sprite>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        sprite.image = hover_image.clone();
                    }
                },
            );
            entity.observe(
                move |event: On<PointerOut>, mut sprites: Query<&mut Sprite>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        sprite.image = default_image.clone();
                    }
                },
            );
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

        // optionally configure the sprite being draggable
        if self.draggable {
            entity.insert(DragState::default());

            entity.observe(
                |event: On<PointerPress>, mut drag_states: Query<&mut DragState>| {
                    if let Ok(mut drag_state) = drag_states.get_mut(event.entity) {
                        drag_state.dragging = true;
                    }
                },
            );
            entity.observe(
                |event: On<PointerRelease>, mut drag_states: Query<&mut DragState>| {
                    if let Ok(mut drag_state) = drag_states.get_mut(event.entity) {
                        drag_state.dragging = false;
                    }
                },
            );
        }

        // optionally enable physics on sprite (gravity and velocity)
        if self.apply_physics {
            entity.insert(physics::PhysicsState::new().enable());
        }
    }
}

#[derive(Component, Default)]
pub struct DragState {
    dragging: bool,
}

// looks for entities with a drag state component and if dragging is enabled then
// have it follow the cursor
pub fn update_dragged_sprites(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut sprites: Query<(&DragState, &mut Transform)>,
) {
    for (drag_state, mut transform) in &mut sprites {
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
            }
        }
    }
}
