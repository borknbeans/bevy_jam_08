use bevy::{asset::Handle, ecs::{bundle::Bundle, observer::{IntoEntityObserver, On}, system::Commands}, image::Image, picking::{Pickable, events::{PointerOut, PointerOver, PointerPress, PointerRelease}}, sprite::Sprite, utils::default};

#[derive(Bundle)]
struct HoverableSprite {
    sprite: Sprite,
    pickable: Pickable,
}

impl HoverableSprite {
    fn new(image: Handle<Image>) -> Self {
        Self {
            sprite: Sprite {
                image,
                ..default()
            },
            pickable: Pickable::default(),
        }
    }
}

// pub fn spawn_hoverable_sprite(mut command: Commands, image: Handle<Image>) {
//     command.spawn(HoverableSprite::new(image))
//         .observe(on_pointer_over)
//         .observe(on_pointer_out)
//         .observe(on_pointer_press)
//         .observe(on_pointer_release);
// }

pub fn spawn_hoverable_sprite(
    mut command: Commands,
    image: Handle<Image>,
    on_over: Option<impl IntoEntityObserver<PointerOver>>,
    on_out: Option<impl IntoEntityObserver<PointerOut>>,
    on_press: Option<impl IntoEntityObserver<PointerPress>>,
    on_release: Option<impl IntoEntityObserver<PointerRelease>>) {
    let mut entity = command.spawn(HoverableSprite::new(image));

    if let Some(observer) = on_over {
        entity.observe(observer);
    } else {
        entity.observe(on_pointer_over);
    }

    if let Some(observer) = on_out {
        entity.observe(observer);
    } else {
        entity.observe(on_pointer_out);
    }

    if let Some(observer) = on_press {
        entity.observe(observer);
    } else {
        entity.observe(on_pointer_press);
    }

    if let Some(observer) = on_release {
        entity.observe(observer);
    } else {
        entity.observe(on_pointer_release);
    }
}

fn on_pointer_over(_: On<PointerOver>) {
    println!("Hover entered");
}

fn on_pointer_out(_: On<PointerOut>) {
    println!("Hover left");
}

fn on_pointer_press(_: On<PointerPress>) {
    println!("Hover pressed");
}

fn on_pointer_release(_: On<PointerRelease>) {
    println!("Hover released");
}
