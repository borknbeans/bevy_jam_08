use bevy::{
    asset::Handle, audio::{AudioPlayer, AudioSource, PlaybackSettings}, camera::Camera, ecs::{
        component::Component, observer::On, query::With, system::{Commands, Query, Res},
    }, image::Image, input::{ButtonInput, ButtonState, mouse::{MouseButton, MouseButtonInput}}, math::Vec3, picking::{
        Pickable,
        events::{PointerOut, PointerOver, PointerPress, PointerRelease},
    }, sprite::Sprite, time::Time, transform::components::{GlobalTransform, Transform}, utils::default, window::{PrimaryWindow, Window},
};
use rand::RngExt;

use crate::physics::{self, PhysicsState};

pub struct ConfigurableSprite {
    default_image: Handle<Image>,
    hover_image: Option<Handle<Image>>,
    press_image: Option<Handle<Image>>,
    hover_sound_fx: Option<Handle<AudioSource>>,
    press_sound_fx: Option<Handle<AudioSource>>,
    bounce_sound_fx: Option<Handle<AudioSource>>,
    draggable: bool,
    apply_physics: bool,
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
                move |event: On<PointerOver>, mut sprites: Query<&mut Sprite>, mut drag_states: Query<&mut DragState>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        if let Ok(drag_state) = drag_states.get_mut(event.entity) {
                            if drag_state.dragging {
                                return;
                            }
                        }
                        sprite.image = hover_image.clone();
                    }
                },
            );
            entity.observe(
                move |event: On<PointerOut>, mut sprites: Query<&mut Sprite>, mut drag_states: Query<&mut DragState>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        if let Ok(drag_state) = drag_states.get_mut(event.entity) {
                            if drag_state.dragging {
                                return;
                            }
                        }
                        sprite.image = default_image.clone();
                    }
                },
            );
        }

        // optionally configure audio playback for hovering 
        if let Some(hover_sound_fx) = self.hover_sound_fx {
            entity.observe(
                move |_: On<PointerOver>, mut commands: Commands| {
                    let speed = rand::rng().random_range(0.9..=1.1);

                    commands.spawn((
                        AudioPlayer::new(hover_sound_fx.clone()),
                        PlaybackSettings::DESPAWN.with_speed(speed),
                    ));
                }
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

        // optionally configure audio playback for pressing
        if let Some(press_sound_fx) = self.press_sound_fx {
            entity.observe(
                move |_: On<PointerPress>, mut commands: Commands| {
                    let speed = rand::rng().random_range(0.9..=1.1);

                    commands.spawn((
                        AudioPlayer::new(press_sound_fx.clone()),
                        PlaybackSettings::DESPAWN.with_speed(speed),
                    ));
                }
            );
        }


        // optionally configure the sprite being draggable
        if self.draggable {
            entity.insert(DragState::default());

            entity.observe(
                |event: On<PointerPress>, mut drag_states: Query<&mut DragState>, mut physics_states: Query<&mut physics::PhysicsState>| {
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

#[derive(Component, Default)]
pub struct DragState {
    dragging: bool,
    pub last_world_position: Vec3
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
                physics_state.set_velocity((transform.translation - drag_state.last_world_position) / time.delta_secs());
                drag_state.last_world_position = transform.translation;
            }
        }
    }
}

pub fn stop_dragging_on_mouse_release(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut draggables: Query<(
        &mut DragState,
        Option<&mut PhysicsState>,
    )>,
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
