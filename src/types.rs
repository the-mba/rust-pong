use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Brick;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Component)]
pub struct Wall(pub WallLocation);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub wall: Wall,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    pub fn new(location: WallLocation, parameters: &Parameters) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position(parameters).extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size(parameters).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: parameters.colors.wall,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
            wall: Wall(location),
        }
    }
}

/// Which side of the arena is this wall located on?
#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum WallLocation {
    Left,
    Right,
    Down,
    Up,
}

impl WallLocation {
    pub fn position(&self, parameters: &Parameters) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(parameters.wall.x_left_wall as f32, 0.),
            WallLocation::Right => Vec2::new(parameters.wall.x_right_wall as f32, 0.),
            WallLocation::Down => Vec2::new(0., parameters.wall.y_down_wall as f32),
            WallLocation::Up => Vec2::new(0., parameters.wall.y_up_wall as f32),
        }
    }

    pub fn size(&self, parameters: &Parameters) -> Vec2 {
        let arena_height = parameters.wall.y_up_wall - parameters.wall.y_down_wall;
        let arena_width = parameters.wall.x_right_wall - parameters.wall.x_left_wall;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0);
        assert!(arena_width > 0);

        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(
                parameters.wall.thickness as f32,
                arena_height as f32 + parameters.wall.thickness as f32,
            ),
            WallLocation::Down | WallLocation::Up => Vec2::new(
                arena_width as f32 + parameters.wall.thickness as f32,
                parameters.wall.thickness as f32,
            ),
        }
    }
}

impl std::fmt::Display for WallLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name = match self {
            WallLocation::Down => "Down",
            WallLocation::Left => "Left",
            WallLocation::Right => "Right",
            WallLocation::Up => "Up",
        };
        write!(f, "{}", name)
    }
}

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    pub scores: Vec<i32>,
}

#[derive(Resource)]
pub struct Speed(pub f32);

#[derive(Clone, Serialize, Deserialize)]
pub enum Effect {
    Move(Vec3),
    Nothing,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Control {
    pub key: MyKeyCode,
    pub effect: Effect,
}

impl Control {
    pub fn new(key: MyKeyCode, effect: Effect) -> Self {
        Self { key, effect }
    }
}
