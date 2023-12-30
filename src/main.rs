//! A simplified implementation of the classic game "Breakout".

pub mod parameters;
use parameters::*;
pub mod types;
use types::*;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use std::iter::zip;

fn main() {
    let parameters = parameters_from_toml();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .insert_resource(Scoreboard {
            scores: vec![0; parameters.players.len()],
        })
        .insert_resource(parameters.clone())
        .insert_resource(Speed(parameters.ball.speed))
        .insert_resource(ClearColor(parameters.colors.background))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddles,
                check_for_collisions,
                play_collision_sound,
                update_velocity,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, (update_scoreboards, bevy::window::close_on_esc))
        .run();
}

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    parameters: Res<Parameters>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // Paddle
    let paddle_x_1 =
        parameters.paddle.left_bound(parameters.as_ref()) + parameters.paddle.width / 2;
    let paddle_x_2 =
        parameters.paddle.right_bound(parameters.as_ref()) - parameters.paddle.width / 2;

    for player in &parameters.players {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(paddle_x_1 as f32, 0.0, 0.0),
                    scale: parameters.paddle.size(),
                    ..default()
                },
                sprite: Sprite {
                    color: parameters.colors.paddle,
                    ..default()
                },
                ..default()
            },
            player.clone(),
            Collider,
        ));
    }

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(parameters.colors.ball)),
            transform: Transform::from_translation(parameters.ball.starting_position)
                .with_scale(parameters.ball.size),
            ..default()
        },
        Ball,
        Velocity(parameters.ball.starting_velocity()),
    ));

    // Scoreboards
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Player 1: ",
                TextStyle {
                    font_size: parameters.scoreboard.font_size,
                    color: parameters.colors.text,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: parameters.scoreboard.font_size,
                color: parameters.colors.score,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: parameters.scoreboard.text_padding,
            left: parameters.scoreboard.text_padding,
            ..default()
        }),
    );
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Player 2: ",
                TextStyle {
                    font_size: parameters.scoreboard.font_size,
                    color: parameters.colors.text,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: parameters.scoreboard.font_size,
                color: parameters.colors.score,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: parameters.scoreboard.text_padding,
            right: parameters.scoreboard.text_padding,
            ..default()
        }),
    );

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left, parameters.as_ref()));
    commands.spawn(WallBundle::new(WallLocation::Right, parameters.as_ref()));
    commands.spawn(WallBundle::new(WallLocation::Down, parameters.as_ref()));
    commands.spawn(WallBundle::new(WallLocation::Up, parameters.as_ref()));

    // Goal Bricks
    let minimum_gap_between_bricks_and_vertical_walls = parameters
        .distribution
        .minimum_gap_between_bricks_and_vertical_walls;
    let minimum_gap_between_bricks_and_horizontal_walls = parameters
        .distribution
        .minimum_gap_between_bricks_and_horizontal_walls;
    let minimum_gap_between_bricks_and_paddle = parameters
        .distribution
        .minimum_gap_between_paddle_and_goal_bricks;

    let y_border_top_wall = parameters.wall.y_up_wall - parameters.wall.thickness / 2;
    let y_border_down_wall = parameters.wall.y_down_wall + parameters.wall.thickness / 2;
    let x_border_left_wall = parameters.wall.x_left_wall + parameters.wall.thickness / 2;
    let x_border_left_paddle = parameters.paddle.left_bound(parameters.as_ref());
    let x_border_right_wall = parameters.wall.x_right_wall + parameters.wall.thickness / 2;
    let x_border_right_paddle = parameters.paddle.right_bound(parameters.as_ref());

    let gap_between_bricks = parameters.distribution.gap_between_bricks;
    let brick_width = parameters.brick.width;
    let total_width_of_bricks = (x_border_left_paddle - x_border_left_wall)
        - minimum_gap_between_bricks_and_vertical_walls
        - minimum_gap_between_bricks_and_paddle;
    let brick_height = parameters.brick.height;
    let total_height_of_bricks = (y_border_top_wall - y_border_down_wall)
        - 2 * minimum_gap_between_bricks_and_horizontal_walls;

    // Goal bricks left
    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns =
        (total_width_of_bricks + gap_between_bricks) / (brick_width + gap_between_bricks);
    let n_rows =
        (total_height_of_bricks + gap_between_bricks) / (brick_height + gap_between_bricks);
    let n_vertical_gaps = n_columns - 1;
    let n_horizontal_gaps = n_rows - 1;

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    let x_left_center_of_bricks = (x_border_left_wall + x_border_left_paddle) / 2;
    let left_left_edge_of_bricks = x_left_center_of_bricks
        // Space taken up by the bricks
        - (n_columns as f32 / 2. * brick_width as f32) as i32
        // Space taken up by the gaps
        - (n_vertical_gaps as f32 / 2. * gap_between_bricks as f32) as i32;
    let x_right_center_of_bricks = (x_border_right_wall + x_border_right_paddle) / 2;
    let right_left_edge_of_bricks = x_right_center_of_bricks
        // Space taken up by the bricks
        - (n_columns as f32 / 2. * brick_width as f32) as i32
        // Space taken up by the gaps
        - (n_vertical_gaps as f32 / 2. * gap_between_bricks as f32) as i32;
    let y_left_center_of_bricks = (y_border_top_wall + y_border_down_wall) / 2;
    let bottom_edge_of_bricks = y_left_center_of_bricks
        // Space taken up by the bricks
        - (n_rows as f32 / 2. * brick_height as f32) as i32
        // Space taken up by the gaps
        - (n_horizontal_gaps as f32 / 2. * gap_between_bricks as f32) as i32;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let left_offset_x = left_left_edge_of_bricks + brick_width / 2;
    let offset_y = bottom_edge_of_bricks + brick_height / 2;
    let right_offset_x = right_left_edge_of_bricks + brick_width / 2;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                (left_offset_x + column * (brick_width + gap_between_bricks)) as f32,
                (offset_y + row * (brick_height + gap_between_bricks)) as f32,
            );

            // brick
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: parameters.colors.brick,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: parameters.brick.size(),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                (right_offset_x + column * (brick_width + gap_between_bricks)) as f32,
                (offset_y + row * (brick_height + gap_between_bricks)) as f32,
            );

            // brick
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: parameters.colors.brick,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: parameters.brick.size(),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }
}

fn move_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    parameters: Res<Parameters>,
) {
    for (mut paddle_transform, player) in zip(query.iter_mut(), &parameters.players) {
        let mut delta: Option<Vec3> = None;
        for this_move in &player.moves {
            if let Effect::Move(direction) = this_move.effect {
                if keyboard_input.pressed(this_move.key.clone().into()) {
                    delta = match delta {
                        Some(delta) => Some(delta + direction),
                        None => Some(direction),
                    }
                }
            }
        }
        if let Some(delta) = delta {
            // Calculate the new horizontal paddle position based on player input
            let new_paddle_position = paddle_transform.translation
                + delta.normalize_or_zero() * parameters.paddle.speed * time.delta_seconds();

            // Update the paddle position,
            // making sure it doesn't cause the paddle to leave the arena
            paddle_transform.translation = new_paddle_position.clamp(
                parameters.paddle.neg_bounds(parameters.as_ref()),
                parameters.paddle.pos_bounds(parameters.as_ref()),
            );
        }
    }
}

fn update_velocity(mut query: Query<&mut Velocity>, speed: Res<Speed>) {
    for mut velocity in &mut query {
        velocity.as_mut().0 = velocity.normalize() * speed.0;
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity, Option<&Ball>)>,
    time: Res<Time>,
    parameters: Res<Parameters>,
) {
    for (mut transform, velocity, maybe_ball) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
        if maybe_ball.is_some() {
            transform.translation = transform.translation.clamp(
                parameters.ball.neg_bounds(parameters.as_ref()),
                parameters.ball.pos_bounds(parameters.as_ref()),
            );
        }
    }
}

fn update_scoreboards(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for (i, mut text) in query.iter_mut().enumerate() {
        if let Some(score) = scoreboard.scores.get(i) {
            text.sections[1].value = score.to_string();
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>, Option<&Wall>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    parameters: Res<Parameters>,
    mut speed: ResMut<Speed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut ball_velocity, ball_transform) in ball_query.iter_mut() {
        let ball_size = ball_transform.scale.truncate();

        // check collision with walls
        for (collider_entity, transform, maybe_brick, maybe_wall) in &collider_query {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                if speed.0 < parameters.ball.max_speed {
                    speed.0 *= 1.01;
                }

                // Sends a collision event so that other systems can react to the collision
                collision_events.send_default();

                if let Some(Wall(wall_hit)) = maybe_wall {
                    for (i, _wall) in parameters
                        .players
                        .iter()
                        .map(|x| &x.wall_that_gives_points)
                        .enumerate()
                    {
                        if wall_hit == _wall {
                            if let Some(scoreboard) = scoreboard.scores.get_mut(i) {
                                *scoreboard += 1;
                                break;
                            }
                        }
                    }
                }

                // Bricks should be despawned and increment the scoreboard on collision
                if maybe_brick.is_some() {
                    commands.entity(collider_entity).despawn();
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }

                if maybe_brick.is_some()
                    && rand::random::<f32>() < parameters.ball.probability_to_duplicate
                {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::default().into()).into(),
                            material: materials.add(ColorMaterial::from(parameters.colors.ball)),
                            transform: Transform::from_translation(
                                parameters.ball.starting_position,
                            )
                            .with_scale(parameters.ball.size),
                            ..default()
                        },
                        Ball,
                        Velocity(ball_velocity.as_ref().0),
                    ));
                }
            }
        }
    }
}

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
