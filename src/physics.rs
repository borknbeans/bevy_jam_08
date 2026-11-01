use bevy::{ecs::{component::Component, system::{Query, Res}}, math::Vec2, time::Time, transform::components::Transform};

#[derive(Component)]
pub struct PhysicsState {
    enabled: bool,
    gravity: f32,
    velocity: Vec2,
}

impl PhysicsState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            gravity: 980.0,
            velocity: Vec2::new(0.0, 0.0),
        }
    }

    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    // pub fn disable(mut self) -> Self {
    //     self.enabled = false;
    //     self
    // }
}

pub fn update_physics_sprites(
    time: Res<Time>,
    mut sprites: Query<(&mut PhysicsState, &mut Transform)>,
) {
    for (mut physics_state, mut transform) in &mut sprites {
        if physics_state.enabled {
            physics_state.velocity.y -= physics_state.gravity * time.delta_secs();

            transform.translation.y += physics_state.velocity.y * time.delta_secs();
        }
    }
}
