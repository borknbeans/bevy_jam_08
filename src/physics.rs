use bevy::{
    camera::Camera,
    ecs::{
        component::Component,
        system::{Query, Res},
    },
    math::Vec2,
    time::Time,
    transform::components::{GlobalTransform, Transform},
};

#[derive(Component)]
pub struct PhysicsState {
    enabled: bool,
    gravity: f32,
    velocity: Vec2,
}

impl PhysicsState {
    pub fn new() -> Self {
        Self {
            enabled: true,
            gravity: 980.0,
            velocity: Vec2::new(0.0, 0.0),
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
}

pub fn update_physics_sprites(
    time: Res<Time>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut sprites: Query<(&mut PhysicsState, &mut Transform)>,
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

        for (mut physics_state, mut transform) in &mut sprites {
            if physics_state.enabled {
                physics_state.velocity.y -= physics_state.gravity * time.delta_secs();

                transform.translation.x += physics_state.velocity.x * time.delta_secs();
                transform.translation.y += physics_state.velocity.y * time.delta_secs();

                if transform.translation.x < tl.x {
                    transform.translation.x = tl.x;
                    if physics_state.velocity.x < 0.0 {
                        physics_state.velocity.x = 0.0;
                    }
                }

                if transform.translation.x > br.x {
                    transform.translation.x = br.x;
                    if physics_state.velocity.x > 0.0 {
                        physics_state.velocity.x = 0.0;
                    }
                }

                if transform.translation.y < br.y {
                    transform.translation.y = br.y;
                    if physics_state.velocity.y < 0.0 {
                        physics_state.velocity.y = 0.0;
                    }
                }

                if transform.translation.y > tl.y {
                    transform.translation.y = tl.y;
                    if physics_state.velocity.y > 0.0 {
                        physics_state.velocity.y = 0.0;
                    }
                }
            }
        }
    }
}
