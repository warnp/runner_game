extern crate glium;
extern crate cgmath;

use engine::vertex::{Vertex, Normal};
use engine::graphic_item::GraphicItem;
use std::u16;
use self::cgmath::prelude::*;
use self::cgmath::{Matrix4,Vector3};

pub trait Model{
    fn get_buffer(&self, display: &glium::Display) ->  (glium::VertexBuffer<Vertex>, glium::VertexBuffer<Normal>, glium::IndexBuffer<u16>);
    fn set_matrix(&self, matrix: Matrix4<f32>);
    fn get_matrix(&self) -> Matrix4<f32>;
}

#[derive(Clone, Debug)]
pub struct Cube {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub normals: Vec<Normal>,
    pub name: String,
    pub matrix: Matrix4<f32>,
}

impl Model for Cube{
    fn get_buffer(&self, display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::VertexBuffer<Normal>, glium::IndexBuffer<u16>) {
        (glium::VertexBuffer::new(display, &self.vertices).unwrap(), glium::VertexBuffer::new(display,&self.normals).unwrap(), glium::IndexBuffer::new(display,glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap())
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
            vertices: vec![
                Vertex {
                    position: (1.0, 1.0, -1.0),
                },
                Vertex {
                    position: (-1.0, 1.0, -1.0),
                },
                Vertex {
                    position: (1.0, 1.0, 1.0),
                },
                Vertex {
                    position: (-1.0, 1.0, 1.0),
                },
                Vertex {
                    position: (-1.0, -1.0, 1.0),
                },
                Vertex {
                    position: (-1.0, 1.0, -1.0),
                },
                Vertex {
                    position: (-1.0, -1.0, -1.0),
                },
                Vertex {
                    position: (1.0, 1.0, -1.0),
                },
                Vertex {
                    position: (1.0, -1.0, -1.0),
                },
                Vertex {
                    position: (1.0, 1.0, 1.0),
                },
                Vertex {
                    position: (1.0, -1.0, 1.0),
                },
                Vertex {
                    position: (-1.0, -1.0, 1.0),
                },
                Vertex {
                    position: (1.0, -1.0, -1.0),
                },
                Vertex {
                    position: (-1.0, -1.0, -1.0),
                }
            ],
            normals: vec![
                Normal{
                    normal: (1.0,0.0,0.0)
                },
                Normal{
                    normal: (1.0,0.0,0.0)
                },
                Normal{
                    normal: (1.0,0.0,0.0)
                },
                Normal{
                    normal: (1.0,0.0,0.0)
                },
                Normal{
                    normal: (1.0,0.0,0.0)
                },
                Normal{
                    normal: (1.0,0.0,0.0)
                },
                Normal{
                    normal: (1.0,0.0,0.0)
                },
                Normal{
                    normal: (1.0,0.0,0.0)
                },
            ],
            indices: vec![ 0, 1, 2, 0, 2, 3,
                           4, 5, 6, 4, 6, 7,
                           3, 2, 5, 3, 5, 4,
                           2, 1, 6, 2, 6, 5,
                           1, 7, 6, 1, 0, 7,
                           0, 3, 4, 0, 4, 7u16
            ],
            name: name,
            matrix: Matrix4::from_translation(Vector3::new(x,y,z)),
        }
    }
}

pub struct Light {
    pub name: String,
    pub  intensity: u8,
    pub position: (f32,f32,f32,f32),
    pub attenuation: (f32,f32,f32),
    pub color: (f32,f32,f32),
    pub radius: f32
}
