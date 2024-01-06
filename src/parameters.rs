use bevy::reflect::Reflect;
use decorum::R32;

const MIN_RGBA_VALUE: f32 = 0.;
const MAX_RGBA_VALUE: f32 = 1.;

mod paddles {
    use tuple_conv::RepeatedTuple as _;

    pub const N: usize = 2;
    pub const WIDTH: Vec<f32> = vec![20.; N];
    pub const HEIGHT: Vec<f32> = vec![120.; N];
    pub const X: Vec<f32> = (-100., 100.).to_vec();
    pub const Y: Vec<f32> = vec![0.; N];
    pub const Z: Vec<f32> = vec![0.; N];
    pub const BOUNDS: Vec<Vec<(f32, f32)>> = (
        ((-100., -100.), (-100., 100.)).to_vec(),
        ((100., -100.), (100., 100.)).to_vec(),
    )
        .to_vec();
    pub const SPEED: Vec<f32> = vec![500.; N];
    pub const COLOR_RGBA: Vec<(f32, f32, f32, f32)> = vec![(0.3, 0.3, 0.7, 1.); N];
    pub const WALL_GIVES_POINTS: Vec<usize> = (0, 2).to_vec();
}

// PADDLES: parameters
//  Bounds: Collection of points, will compute area inside resulting polygon(s). No need to repeat the first point at the end; list will cycle
impl ParametersPaddles {
    fn new_from_const() -> Self {
        let n = paddles::N;
        assert!(if let Some(min_value) = paddles::WIDTH.iter().min() {
            min_value > 0. && !min_value.is_nan() && !min_value.is_infinite()
        });
        assert!(if let Some(min_value) = paddles::HEIGHT.iter().min() {
            min_value > 0. && !min_value.is_nan() && !min_value.is_infinite()
        });
        let width = paddles::WIDTH.iter().map(|e| R32::from(e.abs())).collect();
        let height = paddles::HEIGHT.iter().map(|e| R32::from(e.abs())).collect();
        let x = paddles::X.iter().map(|e| R32::from(e)).collect();
        let y = paddles::Y.iter().map(|e| R32::from(e)).collect();
        let z = paddles::Z.iter().map(|e| R32::from(e)).collect();
        let bounds = paddles::BOUNDS
            .iter()
            .map(|ee| ee.iter().map(|e| R32::from(e)).collect())
            .collect();
        let speed = paddles::SPEED;
        let color_rgba = paddles::COLOR_RGBA;
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

    fn width(&self) -> &Vec<f32> {
        &self.width
    }

    pub fn get_verified() -> Self {
        let s = Self::new();
        // Assert lengths
        for field in s.iter_fields() {
            if let Some(field) = field.downcast_ref::<Vec<f32>>() {
                assert!(field.len() == s.n);
                continue;
            }
            if let Some(field) = field.downcast_ref::<Vec<Vec<(f32, f32)>>>() {
                assert!(field.len() == s.n);
                continue;
            }
            if let Some(field) = field.downcast_ref::<Vec<(f32, f32, f32, f32)>>() {
                assert!(field.len() == s.n);
                continue;
            }
        }
        // Asserting non-zero and positivizing negatives
        s.width = s
            .width
            .iter()
            .map(|e| {
                assert!(*e != 0.);
                e.abs()
            })
            .collect();
        s.height = s
            .height
            .iter()
            .map(|e| {
                assert!(*e != 0.);
                e.abs()
            })
            .collect();
        s.speed = s.speed.iter().map(|e| e.abs()).collect();
        s.color_rgba = s
            .color_rgba
            .iter()
            .map(|ee| {
                ee.to_vec()
                    .iter()
                    .map(|e| e.clamp(MIN_RGBA_VALUE, MAX_RGBA_VALUE))
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        s
    }
}

#[derive(Reflect)]
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
