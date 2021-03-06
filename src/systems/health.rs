use amethyst::renderer::*;
use amethyst::{core::Time, core::Transform, ecs::*};
use std::f32;

use crate::components::health::Health;

pub struct DeathByHealthSystem;

// Entities die if their health reaches zero (or less).
impl<'s> System<'s> for DeathByHealthSystem {
    type SystemData = (ReadStorage<'s, Health>, Entities<'s>);

    fn run(&mut self, (healths, entities): Self::SystemData) {
        for (health, entity) in (&healths, &*entities).join() {
            if health.value < f32::EPSILON {
                let _ = entities.delete(entity);
            }
        }
    }
}

pub struct DebugHealthSystem;

impl<'s> System<'s> for DebugHealthSystem {
    type SystemData = (
        ReadStorage<'s, Health>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (healths, locals, mut debug_lines): Self::SystemData) {
        for (health, local) in (&healths, &locals).join() {
            let pos = local.translation();
            debug_lines.draw_line(
                [pos.x, pos.y + 0.5, 0.0].into(),
                [pos.x + health.value / 100.0, pos.y + 0.5, 0.0].into(),
                [0.0, 1.0, 0.0, 1.0].into(),
            )
        }
    }
}
