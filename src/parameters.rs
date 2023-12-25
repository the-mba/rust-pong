use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::{Control, WallLocation};

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Parameters {
    pub players: Vec<ParametersPlayer>,
    pub paddle: ParametersPaddle,
    pub distribution: ParametersDistribution,
    pub ball: ParametersBall,
    pub wall: ParametersWall,
    pub brick: ParametersBrick,
    pub scoreboard: ParametersScoreboard,
    pub colors: ParametersColors,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParametersPlayer {
    pub wall_that_gives_points: WallLocation,
    pub moves: Vec<Control>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParametersPaddle {
    pub width: i32,
    pub height: i32,
    pub speed: f32,
}

impl ParametersPaddle {
    pub fn size(&self) -> Vec3 {
        Vec3::new(self.width as f32, self.height as f32, 0.)
    }
    pub fn left_bound(&self, parameters: &Parameters) -> i32 {
        parameters.wall.x_left_wall
            + parameters.wall.thickness / 2
            + self.width / 2
            + parameters.distribution.gap_between_paddle_and_side_wall
    }
    pub fn right_bound(&self, parameters: &Parameters) -> i32 {
        parameters.wall.x_right_wall
            - parameters.wall.thickness / 2
            - self.width / 2
            - parameters.distribution.gap_between_paddle_and_side_wall
    }
    pub fn down_bound(&self, parameters: &Parameters) -> i32 {
        parameters.wall.y_down_wall
            + parameters.wall.thickness / 2
            + self.height / 2
            + parameters
                .distribution
                .gap_between_paddle_and_horizontal_wall
    }
    pub fn up_bound(&self, parameters: &Parameters) -> i32 {
        parameters.wall.y_up_wall
            - parameters.wall.thickness / 2
            - self.height / 2
            - parameters
                .distribution
                .gap_between_paddle_and_horizontal_wall
    }
    pub fn neg_bounds(&self, parameters: &Parameters) -> Vec3 {
        Vec3::new(
            self.left_bound(parameters) as f32,
            self.down_bound(parameters) as f32,
            0.,
        )
    }
    pub fn pos_bounds(&self, parameters: &Parameters) -> Vec3 {
        Vec3::new(
            self.right_bound(parameters) as f32,
            self.up_bound(parameters) as f32,
            0.,
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParametersDistribution {
    pub up_direction: Vec3,
    pub down_direction: Vec3,
    pub gap_between_paddle_and_side_wall: i32, // x between paddle and side walls
    pub gap_between_paddle_and_horizontal_wall: i32, // y between paddle and top walls
    pub minimum_gap_between_paddle_and_goal_bricks: i32,
    pub gap_between_bricks: i32,
    pub minimum_gap_between_bricks_and_horizontal_walls: i32,
    pub minimum_gap_between_bricks_and_vertical_walls: i32,
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub size: Vec3,
}

impl ParametersBall {
    pub fn starting_velocity(&self) -> Vec2 {
        self.starting_direction.normalize() * self.speed
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParametersWall {
    pub thickness: i32,
    pub x_left_wall: i32,
    pub x_right_wall: i32,
    pub y_down_wall: i32,
    pub y_up_wall: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ParametersBrick {
    pub width: i32,
    pub height: i32,
}

impl ParametersBrick {
    pub fn size(&self) -> Vec3 {
        Vec3::new(self.width as f32, self.height as f32, 1.)
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
    pub paddle: Color,
    pub ball: Color,
    pub brick: Color,
    pub wall: Color,
    pub text: Color,
    pub score: Color,
}
