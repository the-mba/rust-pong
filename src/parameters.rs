use tuple_conv::RepeatedTuple as _;

// PADDLES: parameters
//  Bounds: Collection of points, will compute area inside resulting polygon(s). No need to repeat the first point at the end; list will cycle
pub const PADDLES_N: usize = 2;

pub const PADDLES_WIDTH: Vec<f32> = vec![20.; PADDLES_N];
pub const PADDLES_HEIGHT: Vec<f32> = vec![120.; PADDLES_N];

pub const PADDLES_X: Vec<f32> = (-100., 100.).to_vec();
pub const PADDLES_Y: Vec<f32> = vec![0.; PADDLES_N];
pub const PADDLES_Z: Vec<f32> = vec![0.; PADDLES_N];

pub const PADDLES_BOUNDS: Vec<Vec<(f32, f32)>> = (
    ((-100., -100.), (-100., 100.)).to_vec(),
    ((100., -100.), (100., 100.)).to_vec(),
)
    .to_vec();

pub const PADDLES_SPEED: Vec<f32> = vec![500.; PADDLES_N];

pub const PADDLES_COLOR_RGBA: Vec<(f32, f32, f32, f32)> = vec![(0.3, 0.3, 0.7, 1.); PADDLES_N];

pub const PADDLES_WALL_THAT_GIVES_POINTS_1: Vec<usize> = (0, 2).to_vec();
