const MISSILE_SIZE: (f32, f32) = (20.0, 5.0);

pub fn create_missile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_pos: Vec2,
    direction: Vec2,
) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 0.0),
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(MISSILE_SIZE.0, MISSILE_SIZE.1)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((Velocity(direction)));
}
