use decorum::R32;

const MIN_RGBA_VALUE: f32 = 0.;
const MAX_RGBA_VALUE: f32 = 1.;

enum VecTypes {
    VecValue(Vec<R32>),
    VecVecValue2Tuple(Vec<Vec<(Value, Value)>>),
    VecValue4Tuple(Vec<(Value, Value, Value, Value)>),
}

#[derive(Clone)]
enum Value {
    Finite(f32),
    NonZero(f32),
}

mod defaults {
    pub mod paddles {
        use super::super::Value::{self, Finite, NonZero};
        use tuple_conv::RepeatedTuple as _;

        const N: usize = 2;
        const WIDTH: Vec<Value> = vec![20.; N];
        const HEIGHT: Vec<Value> = vec![120.; N];
        const X: Vec<Value> = (-100., 100.).to_vec();
        const Y: Vec<Value> = vec![0.; N];
        const Z: Vec<Value> = vec![0.; N];
        const BOUNDS: Vec<Vec<(Value, Value)>> = (
            ((-100., -100.), (-100., 100.)).to_vec(),
            ((100., -100.), (100., 100.)).to_vec(),
        )
            .to_vec();
        const SPEED: Vec<Value> = vec![500.; N];
        const COLOR_RGBA: Vec<(Value, Value, Value, Value)> = vec![(0.3, 0.3, 0.7, 1.); N];
        const WALL_GIVES_POINTS: Vec<usize> = (0, 2).to_vec();
    }
}

// PADDLES: parameters
//  Bounds: Collection of points, will compute area inside resulting polygon(s). No need to repeat the first point at the end; list will cycle
impl ParametersPaddles {
    fn new_verified(s: Self, from_const: bool) -> Self {
        let unverified_n;
        let unverified_width;
        let unverified_height;
        let unverified_x;
        let unverified_y;
        let unverified_z;
        let unverified_bounds;
        let unverified_speed;
        let unverified_color_rgba;
        let unverified_wall_gives_points;

        if from_const {
            unverified_n = defaults::paddles::N;
            unverified_width = defaults::paddles::WIDTH;
            unverified_height = defaults::paddles::HEIGHT;
            unverified_x = defaults::paddles::X;
            unverified_y = defaults::paddles::Y;
            unverified_z = defaults::paddles::Z;
            unverified_bounds = defaults::paddles::BOUNDS;
            unverified_speed = defaults::paddles::SPEED;
            unverified_color_rgba = defaults::paddles::COLOR_RGBA;
            unverified_wall_gives_points = defaults::paddles::WALL_GIVES_POINTS;
        } else {
            unverified_n = s.n;
            unverified_width = s.width;
            unverified_height = s.height;
            unverified_x = s.x;
            unverified_y = s.y;
            unverified_z = s.z;
            unverified_bounds = s.bounds;
            unverified_speed = s.speed;
            unverified_color_rgba = s.color_rgba;
            unverified_wall_gives_points = s.wall_gives_points;
        }

        // n is auto-verified, all other vecs must have .len() == n
        let n = unverified_n;

        assert!(unverified_width.len() == n);
        assert!(unverified_height.len() == n);
        assert!(unverified_x.len() == n);
        assert!(unverified_y.len() == n);
        assert!(unverified_z.len() == n);
        assert!(unverified_bounds.len() == n);
        assert!(unverified_speed.len() == n);
        assert!(unverified_color_rgba.len() == n);
        assert!(unverified_wall_gives_points.len() == n);

        fn verify_non_zero(r: f32) -> f32 {
            assert!(r != 0);
            r
        }

        let width = unverified_width
            .iter()
            .map(|e| R32::from(verify_non_zero(e).abs()))
            .collect();
        let height = unverified_height
            .iter()
            .map(|e| R32::from(verify_non_zero(e).abs()))
            .collect();
        let x = unverified_x.iter().map(|e| R32::from(e)).collect();
        let y = unverified_y.iter().map(|e| R32::from(e)).collect();
        let z = unverified_z.iter().map(|e| R32::from(e)).collect();
        let bounds = paddles::BOUNDS
            .iter()
            .map(|eee| {
                eee.iter()
                    .map(|ee| {
                        ee.to_vec()
                            .iter()
                            .map(|e| R32::from(e))
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect()
            })
            .collect();
        let speed = unverified_speed.iter().map(|e| R32::from(e)).collect();
        let color_rgba = unverified_color_rgba
            .iter()
            .map(|ee| {
                ee.to_vec()
                    .iter()
                    .map(|e| e.clamp(MIN_RGBA_VALUE, MAX_RGBA_VALUE))
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        let wall_gives_points = paddles::WALL_GIVES_POINTS;

        Self {
            n,
            width,
            height,
            x,
            y,
            z,
            bounds,
            speed,
            color_rgba,
            wall_gives_points,
        }
    }
}

pub struct ParametersPaddles {
    pub n: usize,
    pub width: Vec<R32>,
    pub height: Vec<R32>,
    pub x: Vec<R32>,
    pub y: Vec<R32>,
    pub z: Vec<R32>,
    pub bounds: Vec<Vec<(R32, R32)>>,
    pub speed: Vec<R32>,
    pub color_rgba: Vec<(R32, R32, R32, R32)>,
    pub wall_gives_points: Vec<usize>,
}
