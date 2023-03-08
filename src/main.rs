use bevy::{prelude::*, sprite::collide_aabb::collide};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_systems((setup_camera, setup_players, setup_ball))
        .add_systems(
            (
                player_input_system,
                keep_player_inside_bounds,
                ball_movement,
                ball_bounce,
            )
                .chain(),
        )
        .add_system(ball_speed_increase)
        .run();
}

// --- CONSTS --- //

const PLAYER_SIZE: (f32, f32) = (20.0, 100.0);
const PLAYER_SPEED: f32 = 500.0;

const BALL_SIZE: (f32, f32) = (20.0, 20.0);
const BALL_BASE_SPEED: f32 = 100.0;
const BALL_SPEED_INCREASE: f32 = 25.0;

// --- COMPONENTS --- //

#[derive(Component)]
struct Player;

enum PlayerInputType {
    Arrows,
    WASD,
}

#[derive(Component)]
struct PlayerInput(PlayerInputType);

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    speed: f32,
    increase_speed_timer: Timer,
}

// --- SYSTEMS --- //

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_players(mut commands: Commands, windows: Query<&Window>) {
    // Get the window size.
    let window = windows.single();

    let window_size = (window.width(), window.height());
    let window_size_half = (window_size.0 / 2.0, window_size.1 / 2.0);
    let player_size_half = (PLAYER_SIZE.0 / 2.0, PLAYER_SIZE.1 / 2.0);

    // Player 1.
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(-window_size_half.0 + player_size_half.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((Player, PlayerInput(PlayerInputType::WASD)));

    // Player 2.
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(window_size_half.0 - player_size_half.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((Player, PlayerInput(PlayerInputType::Arrows)));
}

fn setup_ball(mut commands: Commands) {
    let ball_size_half = (BALL_SIZE.0 / 2.0, BALL_SIZE.1 / 2.0);

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(-ball_size_half.0, -ball_size_half.1, 0.0),
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(BALL_SIZE.0, BALL_SIZE.1)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: Vec2::new(1.0, 1.0),
            speed: BALL_BASE_SPEED,
            increase_speed_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        });
}

fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&PlayerInput, &mut Transform)>,
) {
    for (player_input, mut transform) in query.iter_mut() {
        let mut direction = 0.0;

        match player_input.0 {
            PlayerInputType::Arrows => {
                if keyboard_input.pressed(KeyCode::Up) {
                    direction += 1.0;
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    direction -= 1.0;
                }
            }
            PlayerInputType::WASD => {
                if keyboard_input.pressed(KeyCode::W) {
                    direction += 1.0;
                }
                if keyboard_input.pressed(KeyCode::S) {
                    direction -= 1.0;
                }
            }
        }

        transform.translation.y += direction * time.delta_seconds() * PLAYER_SPEED;
    }
}

fn keep_player_inside_bounds(
    mut query: Query<&mut Transform, With<Player>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_size = (window.width(), window.height());
    let window_size_half = (window_size.0 / 2.0, window_size.1 / 2.0);
    let player_size_half = (PLAYER_SIZE.0 / 2.0, PLAYER_SIZE.1 / 2.0);

    for mut transform in query.iter_mut() {
        if transform.translation.y > window_size_half.1 - player_size_half.1 {
            transform.translation.y = window_size_half.1 - player_size_half.1;
        }
        if transform.translation.y < -window_size_half.1 + player_size_half.1 {
            transform.translation.y = -window_size_half.1 + player_size_half.1;
        }
    }
}

fn ball_movement(mut query: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut transform, ball) in query.iter_mut() {
        transform.translation += ball.velocity.extend(0.0) * ball.speed * time.delta_seconds();
    }
}

fn ball_speed_increase(mut query: Query<&mut Ball>, time: Res<Time>) {
    for mut ball in query.iter_mut() {
        ball.increase_speed_timer.tick(time.delta());
        if ball.increase_speed_timer.finished() {
            ball.speed += BALL_SPEED_INCREASE;
            println!("Ball speed increased to {}", ball.speed);
        }
    }
}

fn ball_bounce(
    mut query: Query<(&mut Ball, &Transform)>,
    players: Query<&Transform, With<Player>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_size = (window.width(), window.height());
    let window_size_half = (window_size.0 / 2.0, window_size.1 / 2.0);
    let ball_size_half = (BALL_SIZE.0 / 2.0, BALL_SIZE.1 / 2.0);

    for (mut ball, ball_transform) in query.iter_mut() {
        if ball_transform.translation.y > window_size_half.1 - ball_size_half.1 {
            ball.velocity.y = -ball.velocity.y;
        }
        if ball_transform.translation.y < -window_size_half.1 + ball_size_half.1 {
            ball.velocity.y = -ball.velocity.y;
        }

        for player in players.iter() {
            if collide(
                ball_transform.translation,
                Vec2::new(BALL_SIZE.0, BALL_SIZE.1),
                player.translation,
                Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1),
            )
            .is_some()
            {
                ball.velocity.x = -ball.velocity.x;
            }
        }
    }
}
