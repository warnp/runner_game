extern crate glium;
extern crate cgmath;

use std::collections::HashMap;
use engine::vertex::{Vertex, Normal};
use engine::graphic_item::GraphicItem;
use std::u16;
use self::cgmath::prelude::*;
use self::cgmath::{Matrix4, Vector3};
use std::fmt::Debug;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::io::BufReader;
use std::fs::File;



pub trait Model: Debug {
    fn get_buffer(&self, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>);
    fn set_matrix(&self, matrix: Matrix4<f32>);
    fn get_matrix(&self) -> Matrix4<f32>;
}

#[derive(Clone, Debug)]
pub struct Lod {
    pub level: i8,
    pub mesh_name: String,
    pub distance_max: f32,
    pub distance_min: f32,
}

impl Lod {
    pub fn new(mesh_name: String, lod_level: i8, distance_min: f32, distance_max: f32) -> Lod {
        Lod {
            mesh_name: mesh_name,
            distance_max: distance_max,
            distance_min: distance_min,
            level: lod_level,
        }
    }

    pub fn reset_buffer(self) -> Lod {
        Lod {
            mesh_name: self.mesh_name,
            distance_max: self.distance_max,
            distance_min: self.distance_min,
            level: self.level,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    pub lods: HashMap<i8, Lod>,
    pub name: String,
    pub matrix: Matrix4<f32>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub actual_lod: i8
}

impl Model for Cube {
    fn get_buffer(&self, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
        (glium::VertexBuffer::new(display, &self.vertices).unwrap(),
         glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap())
    }
    fn set_matrix(&self, matrix: Matrix4<f32>) {
//        self.matrix = matrix;
    }
    fn get_matrix(&self) -> Matrix4<f32> {
        self.matrix
    }
}

impl Cube {
    pub fn new(name: String,
               x: f32,
               y: f32,
               z: f32,
               color: [f32; 4],
               size: (f32, f32, f32)) -> Cube {
        Cube {
            lods: HashMap::new(),
            name: name,
            matrix: Matrix4::from_nonuniform_scale(size.0, size.1, size.2) *
                Matrix4::from_translation(Vector3::new(x, y, z)),
            indices: vec![],
            vertices: vec![],
            actual_lod: -1
        }
    }

}

pub struct Light {
    pub name: String,
    pub intensity: u8,
    pub position: (f32, f32, f32, f32),
    pub attenuation: (f32, f32, f32),
    pub color: (f32, f32, f32),
    pub radius: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::model::Lod;

    fn should_empty_buffer() {
        let lod = Lod {
            mesh_name: "toto".to_string(),
        };

        let lod = lod.reset_buffer();
        assert!(lod.vertices.len() == 0);
    }
}
