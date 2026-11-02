use bevy::{
    asset::Handle,
    camera::visibility::Visibility,
    ecs::{
        component::Component,
        entity::Entity,
        observer::On,
        query::With,
        system::{Commands, Query, ResMut},
    },
    image::Image,
    math::Vec3,
    picking::{
        Pickable,
        events::{PointerOut, PointerOver, PointerPress},
    },
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};
use smart_default::SmartDefault;

use crate::{
    audio::{AudioBundle, AudioFactory},
    drag::DragState,
    physics::PhysicsState,
    sprite_factory::SpriteFactory,
};

#[derive(Clone, SmartDefault)]
pub struct ConfigurableSprite {
    pub visual_settings: SpriteVisualSettings,
    pub audio_settings: SpriteAudioSettings,
    pub draggable: bool,
    pub apply_physics: bool,
    #[default(true)]
    pub visible: bool,
}

#[derive(Clone, Component, Default)]
pub struct SpriteVisualSettings {
    pub default_img: Handle<Image>,
    pub hover_img: Option<Handle<Image>>,
    pub press_img: Option<Handle<Image>>,
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
    pub fn new(default_img: Handle<Image>) -> Self {
        Self {
            visual_settings: SpriteVisualSettings {
                default_img,
                ..default()
            },
            audio_settings: SpriteAudioSettings::default(),
            draggable: false,
            apply_physics: false,
            visible: true,
        }
    }

    pub fn with_visual_settings(mut self, settings: SpriteVisualSettings) -> Self {
        self.visual_settings = settings;
        self
    }

    pub fn with_audio_settings(mut self, settings: SpriteAudioSettings) -> Self {
        self.audio_settings = settings;
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

    pub fn visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    pub fn spawn_at(
        self,
        commands: &mut Commands,
        spawn_position: Vec3,
        start_dragging: bool,
    ) -> Entity {
        let mut entity = commands.spawn((
            Transform::from_translation(spawn_position),
            Sprite {
                image: if let Some(press_img) = self.visual_settings.press_img.clone()
                    && start_dragging
                {
                    press_img
                } else {
                    self.visual_settings.default_img.clone()
                },
                ..default()
            },
            Pickable::default(),
            self.audio_settings.clone(),
            self.visual_settings.clone(),
        ));

        if !self.visible {
            entity.insert(Visibility::Hidden);
        }

        if !start_dragging {
            entity.insert(Pickable::default());
        }

        // DRAGGING
        if self.draggable {
            entity.insert(DragState {
                dragging: start_dragging,
                last_world_position: spawn_position,
            });

            if start_dragging {
                let mut pickable = Pickable::default();
                pickable.should_block_lower = false;
                entity.insert(pickable);
            }

            // On click, set update drag state and (if applicable) disable physics
            entity.observe(
                |event: On<PointerPress>,
                 mut pickables: Query<&mut Pickable>,
                 mut drag_states: Query<&mut DragState>,
                 mut physics_states: Query<&mut PhysicsState>| {
                    if let Ok(mut pickable) = pickables.get_mut(event.entity) {
                        pickable.should_block_lower = false;
                    }

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

        // PHYSICS
        if self.apply_physics {
            entity.insert(PhysicsState::default());
        }

        // FACTORY
        entity.observe(
            |event: On<PointerPress>,
             factories: Query<(&SpriteFactory, &Transform)>,
             mut commands: Commands| {
                if let Ok((factory, transform)) = factories.get(event.entity) {
                    factory.spawn(&mut commands, transform.translation);
                }
            },
        );

        // VISUAL
        if let Some(hover_img) = self.visual_settings.hover_img.clone() {
            let default_img = self.visual_settings.default_img.clone();

            // Update to hover image
            entity.observe(
                move |event: On<PointerOver>,
                      mut sprites: Query<&mut Sprite>,
                      drag_states: Query<&DragState>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        if let Ok(drag_state) = drag_states.get(event.entity)
                            && drag_state.dragging
                        {
                            return;
                        }
                        sprite.image = hover_img.clone();
                    }
                },
            );

            // Update back to default image
            entity.observe(
                move |event: On<PointerOut>,
                      mut sprites: Query<&mut Sprite>,
                      drag_states: Query<&DragState>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        if let Ok(drag_state) = drag_states.get(event.entity)
                            && drag_state.dragging
                        {
                            return;
                        }
                        sprite.image = default_img.clone();
                    }
                },
            );
        }

        if let Some(press_img) = self.visual_settings.press_img.clone() {
            // Update to press image
            entity.observe(
                move |event: On<PointerPress>, mut sprites: Query<&mut Sprite>| {
                    if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                        sprite.image = press_img.clone();
                    }
                },
            );
            // Updating back is handled in drag.rs when watching for mouse release
        }

        // AUDIO
        if let Some(hover_enter_sound) = self.audio_settings.hover_enter_sound.clone() {
            entity.observe(
                move |_: On<PointerOver>, mut audio_factory: ResMut<AudioFactory>| {
                    audio_factory.play_sound(hover_enter_sound.clone());
                },
            );
        }

        if let Some(hover_exit_sound) = self.audio_settings.hover_exit_sound.clone() {
            entity.observe(
                move |_: On<PointerOut>, mut audio_factory: ResMut<AudioFactory>| {
                    audio_factory.play_sound(hover_exit_sound.clone());
                },
            );
        }

        if let Some(press_sound) = self.audio_settings.press_sound.clone() {
            entity.observe(
                move |_: On<PointerPress>, mut audio_factory: ResMut<AudioFactory>| {
                    audio_factory.play_sound(press_sound.clone());
                },
            );
        }
        entity.id()
    }
}

#[derive(Component)]
pub struct TrashCan;

pub fn update_trash_can_visibility(
    draggables: Query<&DragState>,
    mut trash_cans: Query<&mut Visibility, With<TrashCan>>,
) {
    let something_is_dragging = draggables.iter().any(|state| state.dragging);

    for mut visibility in &mut trash_cans {
        *visibility = if something_is_dragging {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
