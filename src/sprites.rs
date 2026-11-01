use bevy::{asset::Handle, ecs::{bundle::Bundle, observer::{IntoEntityObserver, On}, system::{Commands, Query}}, image::Image, picking::{Pickable, events::{PointerOut, PointerOver, PointerPress, PointerRelease}}, sprite::Sprite, utils::default};

pub struct ConfigurableSprite {
    default_image: Handle<Image>,
    hover_image: Option<Handle<Image>>,
    press_image: Option<Handle<Image>>,
}

impl ConfigurableSprite {
    pub fn new(default_image: Handle<Image>) -> Self {
        Self {
            default_image: default_image,
            hover_image: None,
            press_image: None,
        }
    }

    pub fn with_hover_image(mut self, image: Handle<Image>) -> Self {
        self.hover_image = Some(image);
        self
    }

    pub fn with_press_image(mut self, image: Handle<Image>) -> Self {
        self.press_image = Some(image);
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

        if let Some(hover_image) = self.hover_image {
            let default_image = self.default_image.clone();

            entity.observe(move |event: On<PointerOver>, mut sprites: Query<&mut Sprite>| {
                if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                    sprite.image = hover_image.clone();
                }
            });
            entity.observe(move |event: On<PointerOut>, mut sprites: Query<&mut Sprite>| {
                if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                    sprite.image = default_image.clone();
                }
            });
        }

        if let Some(press_image) = self.press_image {
            let default_image = self.default_image.clone();

            entity.observe(move |event: On<PointerPress>, mut sprites: Query<&mut Sprite>| {
                if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                    sprite.image = press_image.clone();
                }
            });
            entity.observe(move |event: On<PointerRelease>, mut sprites: Query<&mut Sprite>| {
                if let Ok(mut sprite) = sprites.get_mut(event.entity) {
                    sprite.image = default_image.clone();
                }
            });
        }

    }

}
