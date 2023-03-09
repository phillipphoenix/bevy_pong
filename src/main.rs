use bevy::{prelude::*, sprite::collide_aabb::collide};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_systems((setup_camera, setup_players, setup_ball, setup_text))
        .add_systems(
            (
                player_input_system,
                keep_player_inside_bounds,
                ball_movement,
                ball_bounce,
                scoring,
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
struct Player {
    id: i8,
    score: i32,
}

enum PlayerInputType {
    Arrows,
    Wasd,
}

#[derive(Component)]
struct PlayerInput(PlayerInputType);

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    speed: f32,
    increase_speed_timer: Timer,
}

#[derive(Component)]
struct ScoreText {
    player_id: i8,
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
        .insert((
            Player { id: 1, score: 0 },
            PlayerInput(PlayerInputType::Wasd),
        ));

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
        .insert((
            Player { id: 2, score: 0 },
            PlayerInput(PlayerInputType::Arrows),
        ));
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

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/PressStart2P/PressStart2P-Regular.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Player 1 score.
            parent
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new("P1 - ", text_style.clone()),
                        TextSection::new("0", text_style.clone()),
                    ])
                    .with_text_alignment(TextAlignment::Left),
                )
                .insert(ScoreText { player_id: 1 });

            // Player 2 score.
            parent
                .spawn(
                    TextBundle::from_sections([
                        TextSection::new("0", text_style.clone()),
                        TextSection::new(" - P2", text_style.clone()),
                    ])
                    .with_text_alignment(TextAlignment::Right),
                )
                .insert(ScoreText { player_id: 2 });
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
            PlayerInputType::Wasd => {
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

fn scoring(
    mut balls: Query<(&mut Transform, &mut Ball)>,
    mut players: Query<&mut Player>,
    windows: Query<&Window>,
    mut texts: Query<(&mut Text, &ScoreText)>,
) {
    let window = windows.single();
    let window_size = (window.width(), window.height());
    let window_size_half = (window_size.0 / 2.0, window_size.1 / 2.0);

    let (mut ball_transform, mut ball) = balls.single_mut();
    let ball_translation = ball_transform.translation;

    // If ball is out of bounds on the right side, player 1 scores.
    if ball_translation.x > window_size_half.0 + BALL_SIZE.0 {
        for mut player in players.iter_mut() {
            if player.id == 1 {
                player.score += 1;

                for (mut text, score_text) in texts.iter_mut() {
                    if score_text.player_id == player.id {
                        text.sections[1].value = player.score.to_string();
                    }
                }
            }
        }

        // Reset ball position and speed.
        ball.speed = BALL_BASE_SPEED;
        ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
    }

    // If ball is out of bounds on the left side, player 2 scores.
    if ball_translation.x < -window_size_half.0 - BALL_SIZE.0 {
        for mut player in players.iter_mut() {
            if player.id == 2 {
                player.score += 1;

                for (mut text, score_text) in texts.iter_mut() {
                    if score_text.player_id == player.id {
                        text.sections[0].value = player.score.to_string();
                    }
                }
            }
        }

        // Reset ball position and speed.
        ball.speed = BALL_BASE_SPEED;
        ball_transform.translation = Vec3::new(0.0, 0.0, 0.0);
    }
}
