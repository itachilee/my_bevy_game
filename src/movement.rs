use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_spaceship);
    }
}

fn update_spaceship(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
