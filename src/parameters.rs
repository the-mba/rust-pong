use bevy::prelude::*;
use bevy::reflect::Reflect;
use itertools::Itertools as _;
use tuple_conv::RepeatedTuple as _;

const MIN_RGBA_VALUE: f32 = 0.;
const MAX_RGBA_VALUE: f32 = 1.;

// PADDLES: parameters
//  Bounds: Collection of points, will compute area inside resulting polygon(s). No need to repeat the first point at the end; list will cycle
impl ParametersPaddles {
    fn new() -> Self {
        let n: usize = 2;
        Self {
            n,
            width: vec![20.; n],
            height: vec![120.; n],
            x: (-100., 100.).to_vec(),
            y: vec![0.; n],
            z: vec![0.; n],
            bounds: (
                ((-100., -100.), (-100., 100.)).to_vec(),
                ((100., -100.), (100., 100.)).to_vec(),
            )
                .to_vec(),
            speed: vec![500.; n],
            color_rgba: vec![(0.3, 0.3, 0.7, 1.); n],
            wall_gives_points: (0, 2).to_vec(),
        }
    }

    fn width(&self) -> &Vec<f32> {
        &self.width
    }

    fn verify_field<F>(&self, func: F) {}

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
        // Positivizing unnegativazible parameters
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

enum VecTypes {
    VecF32(Vec<f32>),
}

#[derive(Reflect)]
pub struct ParametersPaddles {
    pub n: usize,
    pub width: Vec<f32>,
    pub height: Vec<f32>,
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub z: Vec<f32>,
    pub bounds: Vec<Vec<(f32, f32)>>,
    pub speed: Vec<f32>,
    pub color_rgba: Vec<(f32, f32, f32, f32)>,
    pub wall_gives_points: Vec<usize>,
}
