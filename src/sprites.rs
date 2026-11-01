use bevy::{
    asset::Handle,
    audio::{AudioPlayer, AudioSource, PlaybackSettings},
    ecs::{
        observer::On,
        system::{Commands, Query},
    },
    image::Image,
    picking::{
        Pickable,
        events::{PointerOut, PointerOver, PointerPress, PointerRelease},
    },
    sprite::Sprite,
    utils::default,
};
use rand::RngExt;

use crate::{
    drag::DragState,
    physics::{self, PhysicsState},
};

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
            entity.observe(move |_: On<PointerOver>, mut commands: Commands| {
                let speed = rand::rng().random_range(0.9..=1.1);

                commands.spawn((
                    AudioPlayer::new(hover_sound_fx.clone()),
                    PlaybackSettings::DESPAWN.with_speed(speed),
                ));
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
            entity.observe(move |_: On<PointerPress>, mut commands: Commands| {
                let speed = rand::rng().random_range(0.9..=1.1);

                commands.spawn((
                    AudioPlayer::new(press_sound_fx.clone()),
                    PlaybackSettings::DESPAWN.with_speed(speed),
                ));
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
