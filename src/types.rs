use crate::*;
use serde::{Deserialize, Serialize};

mod states {
    use super::parameters::Level;
    use bevy::prelude::*;

    #[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
    pub enum AppStates {
        #[default]
        Menu,
        Level1(Level),
    }
}
mod events {
    use bevy::prelude::*;
    #[derive(Event)]
    pub struct CollisionEvent;
}

mod resources {
    use bevy::prelude::*;
    #[derive(Resource)]
    pub struct CollisionSound(pub Handle<AudioSource>);
    #[derive(Resource)]
    pub struct Scoreboards {
        pub scores: Vec<f32>,
    }
}
mod parameters {
    use bevy::prelude::*;
    use decorum::R32;
    use itertools::Itertools;
    use serde::{Deserialize, Serialize};
    use std::{fs, io::Write};
    use std::{fs::File, path::Path};
    use toml::to_string;
    use tuple_conv::RepeatedTuple as _;

    use super::components::{Paddle, Player, Wall};

    const PARAMETERS_FILE_PATH: &str = "parameters.toml";
    const ALWAYS_REWRITE_TOML: bool = cfg!(debug_assertions);

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
    pub struct Level {
        pub walls: Vec<Wall>,
        pub paddles: Vec<Paddle>,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Control {
        pub key: MyKeyCode,
        pub effect: Effect,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub enum Effect {
        Move(Vec3),
        Nothing,
    }

    fn r32_tuple_from_vec2(v: &Vec2) -> (R32, R32) {
        (v.x, v.y)
            .to_vec()
            .iter()
            .map(|x| R32::from(*x))
            .collect_tuple()
            .expect("Should be 2 arguments")
    }

    fn r32_tuple_from_color(c: &Color) -> (R32, R32, R32, R32) {
        match *c {
            Color::Rgba { .. } => c.as_rgba_f32(),
            Color::Hsla { .. } => c.as_hsla_f32(),
            Color::Lcha { .. } => c.as_lcha_f32(),
            Color::RgbaLinear { .. } => c.as_linear_rgba_f32(),
        }
        .iter()
        .map(|x| R32::from(*x))
        .collect_tuple()
        .unwrap()
    }

    pub fn wrong_toml(reason: &str) {
        panic!("Unvalid TOML file structure [{path}] ({reason}), delete file and a valid one will be generated.",
                path=PARAMETERS_FILE_PATH, reason=reason)
    }

    pub fn parameters_from_toml() -> Parameters {
        fn write_config_to_file_if_not_exists(
            config: &Parameters,
            file_path: &str,
        ) -> Result<(), Box<dyn std::error::Error>> {
            if !ALWAYS_REWRITE_TOML && Path::new(file_path).exists() {
                return Ok(());
            }
            let toml_string = to_string(config)?;
            let mut file = File::create(file_path)?;
            file.write_all(toml_string.as_bytes())?;
            println!("Config file created successfully.");
            Ok(())
        }

        let parameters = {
            let up_direction = Vec3::new(0., 1., 0.);
            let down_direction = Vec3::new(0., -1., 0.);

            let misc = ParametersMisc {
                minimum_gap_between_paddle_and_goal_bricks: 20.,
                gap_between_bricks: 1.,
                minimum_gap_between_bricks_and_horizontal_walls: 20.,
                minimum_gap_between_bricks_and_vertical_walls: 40.,
            };

            let players = vec![
                Player {
                    controls: vec![
                        Control {
                            key: MyKeyCode::Q,
                            effect: Effect::Move(up_direction),
                        },
                        Control {
                            key: MyKeyCode::A,
                            effect: Effect::Move(down_direction),
                        },
                    ],
                },
                Player {
                    controls: vec![
                        Control {
                            key: MyKeyCode::O,
                            effect: Effect::Move(up_direction),
                        },
                        Control {
                            key: MyKeyCode::L,
                            effect: Effect::Move(down_direction),
                        },
                    ],
                },
            ];

            let ball = ParametersBall {
                starting_position: Vec3::new(0.0, -50.0, -1.0),
                starting_direction: Vec2::new(0.5, -0.5),
                speed: 400.0,
                max_speed: 2000.,
                size: Vec3::new(30.0, 30.0, 0.),
                probability_to_duplicate: 0.1,
                padding_for_bounds: 0.1,
            };

            let levels = {
                // Walls
                let x_left_wall = R32::from(-600.);
                let x_right_wall = R32::from(600.);
                let y_down_wall = R32::from(-300.);
                let y_up_wall = R32::from(300.);
                let thickness = R32::from(10.);

                let walls = vec![
                    Wall {
                        ends: ((x_left_wall, y_up_wall), (x_left_wall, y_down_wall)),
                        thickness,
                    },
                    Wall {
                        ends: ((x_right_wall, y_up_wall), (x_left_wall, y_down_wall)),
                        thickness,
                    },
                    Wall {
                        ends: ((x_left_wall, y_down_wall), (x_right_wall, y_down_wall)),
                        thickness,
                    },
                    Wall {
                        ends: ((x_left_wall, y_up_wall), (x_right_wall, y_up_wall)),
                        thickness,
                    },
                ];

                // Paddles
                let width = 20.;
                let height = 120.;
                // x is dynamic
                let gap_between_paddle_and_vertical_wall = 100.;
                let y = 0.;
                let z = 0.;
                let gap_between_paddle_and_horizontal_wall = 10.;
                let velocity = (R32::from(0.), R32::from(0.));
                let color = Color::rgb(0.3, 0.3, 0.7);

                let y_min = y_down_wall
                    + thickness / 2.
                    + gap_between_paddle_and_horizontal_wall
                    + height / 2.;
                let y_max = y_up_wall
                    - thickness / 2.
                    - gap_between_paddle_and_horizontal_wall
                    - height / 2.;

                let paddle_1 = {
                    let width = width;
                    let height = height;
                    let x = x_left_wall
                        + thickness / 2.
                        + gap_between_paddle_and_vertical_wall
                        + width / 2.;
                    let y = y;
                    let z = z;
                    let (neg_bounds, pos_bounds) = {
                        let x_min = x;
                        let x_max = x;
                        let y_min = y_min;
                        let y_max = y_max;
                        ((x_min, y_min), (x_max, y_max))
                    };
                    let velocity = velocity;
                    let color = color;

                    // Transform into R32
                    let width = R32::from(width);
                    let height = R32::from(height);
                    let x = R32::from(x);
                    let y = R32::from(y);
                    let z = R32::from(z);
                    let neg_bounds = neg_bounds;
                    let pos_bounds = pos_bounds;
                    let velocity = r32_tuple_from_vec2(&velocity);
                    let color = r32_tuple_from_color(&color);

                    Paddle {
                        width,
                        height,
                        x,
                        y,
                        z,
                        neg_bounds,
                        pos_bounds,
                        velocity,
                        color_rgba: color,
                    }
                };
                let paddle_2 = {
                    let width = width;
                    let height = height;
                    let x = x_right_wall
                        - thickness / 2.
                        - gap_between_paddle_and_vertical_wall
                        - width / 2.;
                    let y = y;
                    let z = z;
                    let (neg_bounds, pos_bounds) = {
                        let x_min = x;
                        let x_max = x;
                        let y_min = y_min;
                        let y_max = y_max;
                        ((x_min, y_min), (x_max, y_max))
                    };
                    let velocity = velocity;
                    let color = color;

                    let width = R32::from(width);
                    let height = R32::from(height);
                    let x = R32::from(x);
                    let y = R32::from(y);
                    let z = R32::from(z);
                    let neg_bounds = neg_bounds;
                    let pos_bounds = pos_bounds;
                    let velocity = r32_tuple_from_vec2(&velocity);
                    let color = r32_tuple_from_color(&color);

                    Paddle {
                        width,
                        height,
                        x,
                        y,
                        z,
                        neg_bounds,
                        pos_bounds,
                        velocity,
                        color_rgba: color,
                    }
                };

                let paddles = vec![paddle_1, paddle_2];

                let speed = 500.;
                // Result
                vec![Level { walls, paddles }]
            };

            let brick = ParametersBrick {
                width: 5.,   // was 20
                height: 10., // was 100
            };

            let scoreboard = ParametersScoreboard {
                font_size: 40.0,
                text_padding: Val::Px(5.0),
            };

            let colors = ParametersColors {
                background: Color::rgb(0.9, 0.9, 0.9),
                ball: Color::rgb(1.0, 0.5, 0.5),
                brick: Color::rgb(0.5, 0.5, 1.0),
                wall: Color::rgb(0.8, 0.8, 0.8),
                text: Color::rgb(0.5, 0.5, 1.0),
                score: Color::rgb(1.0, 0.5, 0.5),
            };

            Parameters {
                players,
                misc,
                ball,
                levels,
                brick,
                scoreboard,
                colors,
            }
        };

        let parameters: Parameters = match write_config_to_file_if_not_exists(
            &parameters,
            PARAMETERS_FILE_PATH,
        ) {
            Err(_) => panic!(
                "Couldn't write config to file {} that didn't exist!",
                PARAMETERS_FILE_PATH
            ),
            Ok(_) => {
                let toml_str = fs::read_to_string(PARAMETERS_FILE_PATH)
                    .expect("Failed to read Cargo.toml file, after writing if it didn't exist.");
                toml::from_str(&toml_str).unwrap_or_else(|_| panic!("Unvalid TOML file structure ({}), delete file and a valid one will be generated.",
                PARAMETERS_FILE_PATH))
            }
        };

        parameters
    }

    #[derive(Serialize, Deserialize)]
    pub struct Parameters {
        pub players: Vec<Player>,
        pub misc: ParametersMisc,
        pub ball: ParametersBall,
        pub levels: Vec<Level>,
        pub brick: ParametersBrick,
        pub scoreboard: ParametersScoreboard,
        pub colors: ParametersColors,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct ParametersMisc {
        pub minimum_gap_between_paddle_and_goal_bricks: f32,
        pub gap_between_bricks: f32,
        pub minimum_gap_between_bricks_and_horizontal_walls: f32,
        pub minimum_gap_between_bricks_and_vertical_walls: f32,
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub enum MyKeyCode {
        Key1,
        Key2,
        Key3,
        Key4,
        Key5,
        Key6,
        Key7,
        Key8,
        Key9,
        Key0,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Escape,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,
        Snapshot,
        Scroll,
        Pause,
        Insert,
        Home,
        Delete,
        End,
        PageDown,
        PageUp,
        Left,
        Up,
        Right,
        Down,
        Back,
        Return,
        Space,
        Compose,
        Caret,
        Numlock,
        Numpad0,
        Numpad1,
        Numpad2,
        Numpad3,
        Numpad4,
        Numpad5,
        Numpad6,
        Numpad7,
        Numpad8,
        Numpad9,
        AbntC1,
        AbntC2,
        NumpadAdd,
        Apostrophe,
        Apps,
        Asterisk,
        Plus,
        At,
        Ax,
        Backslash,
        Calculator,
        Capital,
        Colon,
        Comma,
        Convert,
        NumpadDecimal,
        NumpadDivide,
        Equals,
        Grave,
        Kana,
        Kanji,
        AltLeft,
        BracketLeft,
        ControlLeft,
        ShiftLeft,
        SuperLeft,
        Mail,
        MediaSelect,
        MediaStop,
        Minus,
        NumpadMultiply,
        Mute,
        MyComputer,
        NavigateForward,
        NavigateBackward,
        NextTrack,
        NoConvert,
        NumpadComma,
        NumpadEnter,
        NumpadEquals,
        Oem102,
        Period,
        PlayPause,
        Power,
        PrevTrack,
        AltRight,
        BracketRight,
        ControlRight,
        ShiftRight,
        SuperRight,
        Semicolon,
        Slash,
        Sleep,
        Stop,
        NumpadSubtract,
        Sysrq,
        Tab,
        Underline,
        Unlabeled,
        VolumeDown,
        VolumeUp,
        Wake,
        WebBack,
        WebFavorites,
        WebForward,
        WebHome,
        WebRefresh,
        WebSearch,
        WebStop,
        Yen,
        Copy,
        Paste,
        Cut,
    }

    impl From<MyKeyCode> for KeyCode {
        fn from(value: MyKeyCode) -> KeyCode {
            match value {
                MyKeyCode::Key1 => KeyCode::Key1,
                MyKeyCode::Key2 => KeyCode::Key2,
                MyKeyCode::Key3 => KeyCode::Key3,
                MyKeyCode::Key4 => KeyCode::Key4,
                MyKeyCode::Key5 => KeyCode::Key5,
                MyKeyCode::Key6 => KeyCode::Key6,
                MyKeyCode::Key7 => KeyCode::Key7,
                MyKeyCode::Key8 => KeyCode::Key8,
                MyKeyCode::Key9 => KeyCode::Key9,
                MyKeyCode::Key0 => KeyCode::Key0,
                MyKeyCode::A => KeyCode::A,
                MyKeyCode::B => KeyCode::B,
                MyKeyCode::C => KeyCode::C,
                MyKeyCode::D => KeyCode::D,
                MyKeyCode::E => KeyCode::E,
                MyKeyCode::F => KeyCode::F,
                MyKeyCode::G => KeyCode::G,
                MyKeyCode::H => KeyCode::H,
                MyKeyCode::I => KeyCode::I,
                MyKeyCode::J => KeyCode::J,
                MyKeyCode::K => KeyCode::K,
                MyKeyCode::L => KeyCode::L,
                MyKeyCode::M => KeyCode::M,
                MyKeyCode::N => KeyCode::N,
                MyKeyCode::O => KeyCode::O,
                MyKeyCode::P => KeyCode::P,
                MyKeyCode::Q => KeyCode::Q,
                MyKeyCode::R => KeyCode::R,
                MyKeyCode::S => KeyCode::S,
                MyKeyCode::T => KeyCode::T,
                MyKeyCode::U => KeyCode::U,
                MyKeyCode::V => KeyCode::V,
                MyKeyCode::W => KeyCode::W,
                MyKeyCode::X => KeyCode::X,
                MyKeyCode::Y => KeyCode::Y,
                MyKeyCode::Z => KeyCode::Z,
                MyKeyCode::Escape => KeyCode::Escape,
                MyKeyCode::F1 => KeyCode::F1,
                MyKeyCode::F2 => KeyCode::F2,
                MyKeyCode::F3 => KeyCode::F3,
                MyKeyCode::F4 => KeyCode::F4,
                MyKeyCode::F5 => KeyCode::F5,
                MyKeyCode::F6 => KeyCode::F6,
                MyKeyCode::F7 => KeyCode::F7,
                MyKeyCode::F8 => KeyCode::F8,
                MyKeyCode::F9 => KeyCode::F9,
                MyKeyCode::F10 => KeyCode::F10,
                MyKeyCode::F11 => KeyCode::F11,
                MyKeyCode::F12 => KeyCode::F12,
                MyKeyCode::F13 => KeyCode::F13,
                MyKeyCode::F14 => KeyCode::F14,
                MyKeyCode::F15 => KeyCode::F15,
                MyKeyCode::F16 => KeyCode::F16,
                MyKeyCode::F17 => KeyCode::F17,
                MyKeyCode::F18 => KeyCode::F18,
                MyKeyCode::F19 => KeyCode::F19,
                MyKeyCode::F20 => KeyCode::F20,
                MyKeyCode::F21 => KeyCode::F21,
                MyKeyCode::F22 => KeyCode::F22,
                MyKeyCode::F23 => KeyCode::F23,
                MyKeyCode::F24 => KeyCode::F24,
                MyKeyCode::Snapshot => KeyCode::Snapshot,
                MyKeyCode::Scroll => KeyCode::Scroll,
                MyKeyCode::Pause => KeyCode::Pause,
                MyKeyCode::Insert => KeyCode::Insert,
                MyKeyCode::Home => KeyCode::Home,
                MyKeyCode::Delete => KeyCode::Delete,
                MyKeyCode::End => KeyCode::End,
                MyKeyCode::PageDown => KeyCode::PageDown,
                MyKeyCode::PageUp => KeyCode::PageUp,
                MyKeyCode::Left => KeyCode::Left,
                MyKeyCode::Up => KeyCode::Up,
                MyKeyCode::Right => KeyCode::Right,
                MyKeyCode::Down => KeyCode::Down,
                MyKeyCode::Back => KeyCode::Back,
                MyKeyCode::Return => KeyCode::Return,
                MyKeyCode::Space => KeyCode::Space,
                MyKeyCode::Compose => KeyCode::Compose,
                MyKeyCode::Caret => KeyCode::Caret,
                MyKeyCode::Numlock => KeyCode::Numlock,
                MyKeyCode::Numpad0 => KeyCode::Numpad0,
                MyKeyCode::Numpad1 => KeyCode::Numpad1,
                MyKeyCode::Numpad2 => KeyCode::Numpad2,
                MyKeyCode::Numpad3 => KeyCode::Numpad3,
                MyKeyCode::Numpad4 => KeyCode::Numpad4,
                MyKeyCode::Numpad5 => KeyCode::Numpad5,
                MyKeyCode::Numpad6 => KeyCode::Numpad6,
                MyKeyCode::Numpad7 => KeyCode::Numpad7,
                MyKeyCode::Numpad8 => KeyCode::Numpad8,
                MyKeyCode::Numpad9 => KeyCode::Numpad9,
                MyKeyCode::AbntC1 => KeyCode::AbntC1,
                MyKeyCode::AbntC2 => KeyCode::AbntC2,
                MyKeyCode::NumpadAdd => KeyCode::NumpadAdd,
                MyKeyCode::Apostrophe => KeyCode::Apostrophe,
                MyKeyCode::Apps => KeyCode::Apps,
                MyKeyCode::Asterisk => KeyCode::Asterisk,
                MyKeyCode::Plus => KeyCode::Plus,
                MyKeyCode::At => KeyCode::At,
                MyKeyCode::Ax => KeyCode::Ax,
                MyKeyCode::Backslash => KeyCode::Backslash,
                MyKeyCode::Calculator => KeyCode::Calculator,
                MyKeyCode::Capital => KeyCode::Capital,
                MyKeyCode::Colon => KeyCode::Colon,
                MyKeyCode::Comma => KeyCode::Comma,
                MyKeyCode::Convert => KeyCode::Convert,
                MyKeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
                MyKeyCode::NumpadDivide => KeyCode::NumpadDivide,
                MyKeyCode::Equals => KeyCode::Equals,
                MyKeyCode::Grave => KeyCode::Grave,
                MyKeyCode::Kana => KeyCode::Kana,
                MyKeyCode::Kanji => KeyCode::Kanji,
                MyKeyCode::AltLeft => KeyCode::AltLeft,
                MyKeyCode::BracketLeft => KeyCode::BracketLeft,
                MyKeyCode::ControlLeft => KeyCode::ControlLeft,
                MyKeyCode::ShiftLeft => KeyCode::ShiftLeft,
                MyKeyCode::SuperLeft => KeyCode::SuperLeft,
                MyKeyCode::Mail => KeyCode::Mail,
                MyKeyCode::MediaSelect => KeyCode::MediaSelect,
                MyKeyCode::MediaStop => KeyCode::MediaStop,
                MyKeyCode::Minus => KeyCode::Minus,
                MyKeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
                MyKeyCode::Mute => KeyCode::Mute,
                MyKeyCode::MyComputer => KeyCode::MyComputer,
                MyKeyCode::NavigateForward => KeyCode::NavigateForward,
                MyKeyCode::NavigateBackward => KeyCode::NavigateBackward,
                MyKeyCode::NextTrack => KeyCode::NextTrack,
                MyKeyCode::NoConvert => KeyCode::NoConvert,
                MyKeyCode::NumpadComma => KeyCode::NumpadComma,
                MyKeyCode::NumpadEnter => KeyCode::NumpadEnter,
                MyKeyCode::NumpadEquals => KeyCode::NumpadEquals,
                MyKeyCode::Oem102 => KeyCode::Oem102,
                MyKeyCode::Period => KeyCode::Period,
                MyKeyCode::PlayPause => KeyCode::PlayPause,
                MyKeyCode::Power => KeyCode::Power,
                MyKeyCode::PrevTrack => KeyCode::PrevTrack,
                MyKeyCode::AltRight => KeyCode::AltRight,
                MyKeyCode::BracketRight => KeyCode::BracketRight,
                MyKeyCode::ControlRight => KeyCode::ControlRight,
                MyKeyCode::ShiftRight => KeyCode::ShiftRight,
                MyKeyCode::SuperRight => KeyCode::SuperRight,
                MyKeyCode::Semicolon => KeyCode::Semicolon,
                MyKeyCode::Slash => KeyCode::Slash,
                MyKeyCode::Sleep => KeyCode::Sleep,
                MyKeyCode::Stop => KeyCode::Stop,
                MyKeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
                MyKeyCode::Sysrq => KeyCode::Sysrq,
                MyKeyCode::Tab => KeyCode::Tab,
                MyKeyCode::Underline => KeyCode::Underline,
                MyKeyCode::Unlabeled => KeyCode::Unlabeled,
                MyKeyCode::VolumeDown => KeyCode::VolumeDown,
                MyKeyCode::VolumeUp => KeyCode::VolumeUp,
                MyKeyCode::Wake => KeyCode::Wake,
                MyKeyCode::WebBack => KeyCode::WebBack,
                MyKeyCode::WebFavorites => KeyCode::WebFavorites,
                MyKeyCode::WebForward => KeyCode::WebForward,
                MyKeyCode::WebHome => KeyCode::WebHome,
                MyKeyCode::WebRefresh => KeyCode::WebRefresh,
                MyKeyCode::WebSearch => KeyCode::WebSearch,
                MyKeyCode::WebStop => KeyCode::WebStop,
                MyKeyCode::Yen => KeyCode::Yen,
                MyKeyCode::Copy => KeyCode::Copy,
                MyKeyCode::Paste => KeyCode::Paste,
                MyKeyCode::Cut => KeyCode::Cut,
            }
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct ParametersBall {
        pub starting_position: Vec3,
        pub starting_direction: Vec2,
        pub speed: f32,
        pub max_speed: f32,
        pub size: Vec3,
        pub probability_to_duplicate: f32,
        pub padding_for_bounds: f32,
    }

    impl ParametersBall {
        pub fn starting_velocity(&self) -> Vec2 {
            self.starting_direction.normalize() * self.speed
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct ParametersBrick {
        pub width: f32,
        pub height: f32,
    }

    impl ParametersBrick {
        pub fn size(&self) -> Vec3 {
            Vec3::new(self.width, self.height, 1.)
        }
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct ParametersScoreboard {
        pub font_size: f32,
        pub text_padding: Val,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct ParametersColors {
        pub background: Color,
        pub ball: Color,
        pub brick: Color,
        pub wall: Color,
        pub text: Color,
        pub score: Color,
    }
}

mod components {
    use bevy::prelude::*;
    use decorum::R32;
    use itertools::Itertools;
    use serde::{Deserialize, Serialize};
    use tuple_conv::RepeatedTuple as _;

    use super::parameters::Control;

    #[derive(Clone, Serialize, Deserialize, Component)]
    pub struct Player {
        pub controls: Vec<Control>,
    }

    #[derive(Component)]
    pub struct Ball;

    #[derive(Component, Deref, DerefMut, Debug)]
    pub struct Velocity(pub Vec2);

    #[derive(Component)]
    pub struct Collider;

    #[derive(Component)]
    pub struct Brick;

    #[derive(Debug, Copy, Clone, Serialize, Deserialize, Component, Eq, PartialEq, Hash)]
    pub struct Paddle {
        pub width: R32,
        pub height: R32,
        pub x: R32,
        pub y: R32,
        pub z: R32,
        pub neg_bounds: (R32, R32),
        pub pos_bounds: (R32, R32),
        pub velocity: (R32, R32),
        pub color_rgba: (R32, R32, R32, R32),
    }

    fn vec3_from_r32_tuple(r32_tuple: &(R32, R32, R32)) -> Vec3 {
        let a = r32_tuple.0.into_inner();
        let r32 = r32_tuple
            .to_vec()
            .iter()
            .map(|x| x.into_inner())
            .collect::<Vec<f32>>();
        let mut my_array: [f32; 3];
        my_array.iter_mut().set_from(r32);
        Vec3::from_array(my_array)
    }

    impl Paddle {
        pub fn position(&self) -> Vec3 {
            vec3_from_r32_tuple(&(self.x, self.y, self.z))
        }
        pub fn size(&self) -> Vec3 {
            vec3_from_r32_tuple(&(self.width, self.height, R32::from(0.)))
        }
    }
    #[derive(Debug, Clone, Component, Serialize, Deserialize, Eq, PartialEq, Hash)]
    pub struct Wall {
        pub ends: ((R32, R32), (R32, R32)),
        pub thickness: R32,
    }
}

mod bundles {
    use bevy::prelude::*;

    #[derive(Bundle)]
    pub struct PlayerBundle {
        pub sprite_bundle: SpriteBundle,
        pub collider: Collider,
        pub player: Player,
    }

    impl PlayerBundle {
        pub fn new(player: &Player, translation: Vec3, color: Color) -> Self {
            Self {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation,
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
}
