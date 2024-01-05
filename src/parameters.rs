use bevy::prelude::*;
use bevy::reflect::{Reflect, ReflectRef};
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
        let l = s.x.len();

        let info = s.get_info();
        let reflected = reflect_component.reflect(s).unwrap();
        let ReflectRef::Struct(reflected) = reflected.reflect_ref() else {
            unreachable!()
        };
        /* for (i, field) in reflected.iter_fields().enumerate() {
            ui.label(format!(
                "{:?} : {:?}",
                reflected.name_at(i).unwrap(),
                reflected.field_at(i).unwrap()
            ));
        } */

        for (i, _) in s.iter_fields().enumerate() {
            /* if let Vec { buf, len } = field.downcast_ref().unwrap() {
                continue;
            }
            if let Some(field) = field.downcast_ref::<Vec<f32>>() {
                //.downcast_ref::<Vec<f32>>()
                assert!(field.len() == s.n);
            } */
            let f = s.field_at(i).unwrap().len();
        }

        s
    }
}

enum VecTypes {
    VecF32(Vec<f32>),
}

#[derive(FromReflect, Reflect, Component, Default)]
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
