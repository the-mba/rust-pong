pub mod states {
    use super::parameters::Level;
    use bevy::prelude::*;

    #[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
    pub enum AppStates {
        #[default]
        Menu,
        Level1(Level),
    }
}

pub mod events {
    use bevy::prelude::*;
    #[derive(Event)]
    pub struct CollisionEvent;
}

pub mod resources {
    use bevy::prelude::*;
    #[derive(Resource)]
    pub struct CollisionSound(pub Handle<AudioSource>);
    #[derive(Resource)]
    pub struct Scoreboards {
        pub scores: Vec<f32>,
    }
}

pub mod components {
    use std::slice::Iter;

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

    #[derive(Debug, Clone, Serialize, Deserialize, Component, Eq, PartialEq, Hash)]
    pub struct Paddle {
        pub width: R32,
        pub height: R32,
        pub x: R32,
        pub y: R32,
        pub z: R32,
        pub bounds: Vec<(R32, R32)>,
        pub speed: R32,
        pub color_rgba: (R32, R32, R32, R32),
        pub wall_that_gives_points: usize,
    }

    fn vec3_from_r32_tuple(r32_tuple: &(R32, R32, R32)) -> Vec3 {
        let a = r32_tuple.0.into_inner();
        let r32 = r32_tuple
            .to_vec()
            .iter()
            .map(|e| e.into_inner())
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
        pub fn speed(&self) -> f32 {
            self.speed.into_inner()
        }
        pub fn color(&self) -> Color {
            Color::rgba(
                self.color_rgba.0.into_inner(),
                self.color_rgba.1.into_inner(),
                self.color_rgba.2.into_inner(),
                self.color_rgba.3.into_inner(),
            )
        }
    }

    #[derive(Debug, Clone, Component, Serialize, Deserialize, Eq, PartialEq, Hash)]
    pub struct Wall {
        pub ends: ((R32, R32), (R32, R32)),
        pub thickness: R32,
        pub color: (R32, R32, R32, R32),
    }

    impl Wall {
        pub fn end_a(&self) -> Vec2 {
            Wall::vec2_from_r32_2tuple(&self.ends.0)
        }
        pub fn end_b(&self) -> Vec2 {
            Wall::vec2_from_r32_2tuple(&self.ends.1)
        }
        pub fn thickness(&self) -> f32 {
            self.thickness.into_inner()
        }
        pub fn translation(&self) -> Vec2 {
            (self.end_a() + self.end_b()) / 2.
        }
        pub fn scale(&self) -> Vec2 {
            let end_a = self.end_a();
            let end_b = self.end_b();
            let dir_a_to_b = (end_b - end_a).normalize_or_zero();
            let left = dir_a_to_b.perp();
            let right = dir_a_to_b.perp().perp().perp();
            let bottom_left = end_a + (left * self.thickness());
            let bottom_right = end_a + (right * self.thickness());
            let top_left = end_b + (left * self.thickness());
            let top_right = end_b + (right * self.thickness());
            let vertices = (bottom_left, bottom_right, top_left, top_right).to_vec();

            enum Coordinates {
                X = 0,
                Y = 1,
            }

            fn get_extreme<F>(vertices: Vec<Vec2>, func: F, coord: Coordinates) -> f32
            where
                F: Fn(Iter<R32>) -> Option<&R32>,
            {
                let l = vertices.len();
                assert!(l > 0);
                assert!((coord as usize) < l);

                func(
                    vertices
                        .iter()
                        .map(|e| {
                            R32::from(
                                *e.iter_fields()
                                    .nth(coord as usize)
                                    .unwrap()
                                    .downcast_ref::<f32>()
                                    .unwrap(),
                            )
                        })
                        .collect::<Vec<R32>>()
                        .iter(),
                )
                .unwrap()
                .into_inner()
            }

            let x_min = get_extreme(vertices, Iterator::min, Coordinates::X);
            let x_max = get_extreme(vertices, Iterator::max, Coordinates::X);
            let y_min = get_extreme(vertices, Iterator::min, Coordinates::Y);
            let y_max = get_extreme(vertices, Iterator::max, Coordinates::Y);

            let x_delta = x_max - x_min;
            let y_delta = y_max - y_min;

            Vec2::new(x_delta, y_delta)
        }
        fn vec2_from_r32_2tuple(r32_tuple: &(R32, R32)) -> Vec2 {
            let r32: Vec<f32> = r32_tuple.to_vec().iter().map(|e| e.into_inner()).collect();
            let mut my_array: [f32; 2];
            my_array.iter_mut().set_from(r32);
            Vec2::from_array(my_array)
        }
    }
}

pub mod bundles {
    use bevy::prelude::*;

    use super::components::{Collider, Player, Wall};

    #[derive(Bundle)]
    pub struct PlayerBundle {
        pub sprite_bundle: SpriteBundle,
        pub collider: Collider,
        pub player: Player,
    }

    impl PlayerBundle {
        pub fn new(player: &Player, translation: Vec3, scale: Vec3, color: Color) -> Self {
            Self {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation,
                        scale,
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
        pub fn new(wall: &Wall) -> WallBundle {
            let wall = *wall;
            let translation = wall.translation().extend(0.);
            let scale = wall.scale().extend(1.);
            let color = wall.color;
            let color = (
                color.0.into_inner(),
                color.1.into_inner(),
                color.2.into_inner(),
                color.3.into_inner(),
            );
            let color = Color::rgba(color.0, color.1, color.2, color.3);
            WallBundle {
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                        // This is used to determine the order of our sprites
                        translation,
                        // The z-scale of 2D objects must always be 1.0,
                        // or their ordering will be affected in surprising ways.
                        // See https://github.com/bevyengine/bevy/issues/4149
                        scale,
                        ..default()
                    },
                    sprite: Sprite { color, ..default() },
                    ..default()
                },
                collider: Collider,
                wall,
            }
        }
    }
}

pub mod parameters {
    use bevy::prelude::*;
    use decorum::R32;
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
                let x_left_wall = -600.;
                let x_right_wall = 600.;
                let y_down_wall = -300.;
                let y_up_wall = 300.;
                let thickness = 10.;
                let color: (f32, f32, f32, f32) = (0.8, 0.8, 0.8, 1.);

                let x_left_wall = R32::from(x_left_wall);
                let x_right_wall = R32::from(x_right_wall);
                let y_down_wall = R32::from(y_down_wall);
                let y_up_wall = R32::from(y_up_wall);
                let thickness = R32::from(thickness);
                let color = (
                    R32::from(color.0),
                    R32::from(color.1),
                    R32::from(color.2),
                    R32::from(color.3),
                );

                let walls = vec![
                    Wall {
                        ends: ((x_left_wall, y_up_wall), (x_left_wall, y_down_wall)),
                        thickness,
                        color,
                    },
                    Wall {
                        ends: ((x_right_wall, y_up_wall), (x_left_wall, y_down_wall)),
                        thickness,
                        color,
                    },
                    Wall {
                        ends: ((x_left_wall, y_down_wall), (x_right_wall, y_down_wall)),
                        thickness,
                        color,
                    },
                    Wall {
                        ends: ((x_left_wall, y_up_wall), (x_right_wall, y_up_wall)),
                        thickness,
                        color,
                    },
                ];

                // Paddles
                let width = 20.;
                let height = 120.;
                // x is dynamic
                let y: f32 = 0.;
                let z = 0.;
                let speed = 0.;
                let color_rgba = (0.3, 0.3, 0.7, 1.);

                fn paddle(
                    width: f32,
                    height: f32,
                    x: f32,
                    y: f32,
                    z: f32,
                    bounds: Vec<(f32, f32)>,
                    speed: f32,
                    color_rgba: (f32, f32, f32, f32),
                    wall_that_gives_points: usize,
                ) -> Paddle {
                    // Transform into R32, so we can Serialize, Deserialize and have Eq (required by trait States)
                    let width = R32::from(width);
                    let height = R32::from(height);
                    let x = R32::from(x);
                    let y = R32::from(y);
                    let z = R32::from(z);
                    let bounds = bounds
                        .to_vec()
                        .iter()
                        .map(|e| (R32::from(e.0), R32::from(e.1)))
                        .collect::<Vec<(R32, R32)>>();
                    let speed = R32::from(speed);
                    let color_rgba = (
                        R32::from(color_rgba.0),
                        R32::from(color_rgba.1),
                        R32::from(color_rgba.2),
                        R32::from(color_rgba.3),
                    );

                    Paddle {
                        width,
                        height,
                        x,
                        y,
                        z,
                        bounds,
                        speed,
                        color_rgba,
                        wall_that_gives_points,
                    }
                }

                let x_1 = -100.;
                let bounds_1 = ((-100., -100.), (-100., 100.)).to_vec();
                let wall_that_gives_points_1 = 0;
                let paddle_1 = paddle(
                    width,
                    height,
                    x_1,
                    y,
                    z,
                    bounds_1,
                    speed,
                    color_rgba,
                    wall_that_gives_points_1,
                );

                let paddle_2 = {
                    let wall_that_gives_points = 0;

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
                    let speed = speed;
                    let color = color;

                    Paddle {
                        width,
                        height,
                        x,
                        y,
                        z,
                        neg_bounds,
                        pos_bounds,
                        speed,
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
        pub text: Color,
        pub score: Color,
    }
}

pub mod regular_polygon {
    use bevy::render::{
        mesh::{Indices, Mesh},
        render_resource::PrimitiveTopology,
    };

    /// A regular polygon in the `XY` plane
    #[derive(Debug, Copy, Clone)]
    pub struct RegularPolygon {
        /// Circumscribed radius in the `XY` plane.
        ///
        /// In other words, the vertices of this polygon will all touch a circle of this radius.
        pub radius: f32,
        /// Number of sides.
        pub sides: usize,
    }

    impl Default for RegularPolygon {
        fn default() -> Self {
            Self {
                radius: 0.5,
                sides: 6,
            }
        }
    }

    impl RegularPolygon {
        /// Creates a regular polygon in the `XY` plane
        pub fn new(radius: f32, sides: usize) -> Self {
            Self { radius, sides }
        }
    }

    impl From<RegularPolygon> for Mesh {
        fn from(polygon: RegularPolygon) -> Self {
            let RegularPolygon { radius, sides } = polygon;

            debug_assert!(sides > 2, "RegularPolygon requires at least 3 sides.");

            let mut positions = Vec::with_capacity(sides);
            let mut normals = Vec::with_capacity(sides);
            let mut uvs = Vec::with_capacity(sides);

            let step = std::f32::consts::TAU / sides as f32;
            for i in 0..sides {
                let theta = std::f32::consts::FRAC_PI_2 - i as f32 * step;
                let (sin, cos) = theta.sin_cos();

                positions.push([cos * radius, sin * radius, 0.0]);
                normals.push([0.0, 0.0, 1.0]);
                uvs.push([0.5 * (cos + 1.0), 1.0 - 0.5 * (sin + 1.0)]);
            }

            let mut indices = Vec::with_capacity((sides - 2) * 3);
            for i in 1..(sides as u32 - 1) {
                indices.extend_from_slice(&[0, i + 1, i]);
            }

            Mesh::new(PrimitiveTopology::TriangleList)
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
                .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
                .with_indices(Some(Indices::U32(indices)))
        }
    }
}
