use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_spaceship);
    }
}

fn print_spaceship(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!(
            "entity  {:?} at {} {} {}",
            entity, transform.translation.x, transform.translation.y, transform.translation.z
        );
    }
}
