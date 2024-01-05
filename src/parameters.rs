use bevy::prelude::*;
use bevy::reflect::Reflect;
use tuple_conv::RepeatedTuple as _;

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

    pub fn get_verified() -> Self {
        let s = Self::new();
        let n = s.n;
        for field in s.iter_fields() {
            assert!(field.downcast_ref::<std::vec::Vec<_>>().unwrap().len() == n);
        }

        assert!(s.width.len() == n);
        assert!(s.height.len() == n);
        assert!(s.x.len() == n);
        assert!(s.y.len() == n);
        assert!(s.z.len() == n);
        assert!(s.bounds.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);
        assert!(s.width.len() == n);

        s
    }
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
