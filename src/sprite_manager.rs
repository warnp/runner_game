use sprite::Sprite;

extern crate glium;
use glium::backend::Facade;
use glium::{DisplayBuild, Surface};

use vertex;

#[derive(Debug)]
pub struct SpriteManager<'a> {
    sprite_list: Vec<Sprite<'a>>,
}

impl <'a>SpriteManager<'a> {
    fn new(&self,sprites: Vec<Sprite<'a>>) -> SpriteManager {
        self.sprite_list = sprites;
    }

    fn get_vertex_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> Result<glium::VertexBuffer<vertex::Vertex>, glium::vertex::BufferCreationError> {
        let vertex_list = Vec::new();
        for s in &self.sprite_list{
            for v in &s.vertices {
                vertex_list.push(v);
            }
        }

        glium::VertexBuffer::new(display, &vertex_list.slice(0,vertex_list.len()))
    }

    fn get_index_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> Result<glium::IndexBuffer<u16>, glium::index::BufferCreationError> {
        let index_list = Vec::new();

//a revoir
        for s in &self.sprite_list {
            let array_size = &self.sprite_list.len();
            for (iterator,i) in &s.indices {
                index_list.push(i + iterator * array_size);
            }
        }
        glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, index_list)
    }
}
