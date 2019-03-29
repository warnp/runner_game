extern crate glium;
extern crate cgmath;

use std::collections::HashMap;
use engine::graphic::vertex::{Vertex};
use std::u16;
use self::cgmath::{Matrix4};
use std::fmt::Debug;


pub trait Model: Debug {
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_buffer(&self, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>);
    fn set_matrix(&mut self, matrix: Matrix4<f32>);
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
pub struct StaticMesh {
    pub id_name: String,
    pub lods: HashMap<i8, Lod>,
    pub name: String,
    pub matrix: Matrix4<f32>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub actual_lod: i8,

}

impl PartialEq for StaticMesh {
    fn eq(&self, other: &StaticMesh) -> bool {
        self.name == other.name
    }
}

impl Model for StaticMesh {
    fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }
    fn get_buffer(&self, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
        (glium::VertexBuffer::new(display, &self.vertices).unwrap(),
         glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap())
    }
    fn set_matrix(&mut self, matrix: Matrix4<f32>) {
        self.matrix = matrix;
    }
    fn get_matrix(&self) -> Matrix4<f32> {
        self.matrix
    }
}

impl StaticMesh {
    pub fn new(
                id_name: String,
                name: String,
               matrix: Matrix4<f32>,
               color: [f32; 4]) -> StaticMesh {

        StaticMesh {
            id_name: id_name,
            lods: HashMap::new(),
            name: name,
            matrix: matrix,
            indices: vec![],
            vertices: vec![],
            actual_lod: -1,

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
    pub direction: (f32,f32,f32),
    pub is_distant: bool,
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
