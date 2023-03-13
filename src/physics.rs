use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn velocity_move(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}
