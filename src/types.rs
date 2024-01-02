use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppStates {
    #[default]
    Menu,
    Level1(Level),
}

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Brick;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub player: Player,
}

impl PlayerBundle {
    pub fn new(player: &Player, x: i32, color: Color) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x as f32, 0.0, 0.0),
                    scale: player.paddle.size(),
                    ..default()
                },
                sprite: Sprite { color, ..default() },
                ..default()
            },
            collider: Collider,
            player: player.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Component)]
pub struct Player {
    pub wall_that_gives_points: usize,
    pub controls: Vec<Control>,
    pub paddle: Paddle,
}

impl Player {
    pub fn new_bundle(&self) -> (SpriteBundle, Paddle, Collider) {
        (
            SpriteBundle {
                transform: Transform {
                    translation: self.paddle.position(),
                    scale: Vec3::new(self.paddle.width, self.paddle.height, 10.),
                    ..default()
                },
                sprite: Sprite {
                    color: self.paddle.color,
                    ..default()
                },
                ..default()
            },
            Paddle {},
            Collider,
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Component)]
pub struct Paddle {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub position: 
    pub color: Color,
}

impl Paddle {
    pub fn position(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn size(&self) -> Vec3 {
        Vec3::new(self.width, self.height, 0.)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Level {
    pub walls: Vec<Wall>,
    pub paddles: Vec<Paddle>,
}

#[derive(Component)]
pub struct Wall {
    pub position: Vec3,
    pub end_a: Vec2,
    pub end_b: Vec2,
    pub thickness: i32,
}

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

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboards {
    pub cndn: Vec<i32>,
    pub scores: Vec<i32>,
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

#[derive(Clone, Serialize, Deserialize)]
pub enum Effect {
    Move(Vec3),
    Nothing,
}
