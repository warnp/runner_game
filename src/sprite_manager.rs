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

    fn get_vertex_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> glium::VertexBuffer<vertex::Vertex>{

        let mut vb : glium::VertexBuffer<vertex::Vertex> = glium::VertexBuffer::empty_dynamic(display, self.sprite_list.len() * 4).unwrap();

        for (num, sprite) in vb.map().chunks_mut(4).enumerate() {
            sprite[0].position[0] = self.sprite_list[num * 4].vertices[0].position[0];
            sprite[0].position[1] = self.sprite_list[num * 4].vertices[0].position[1];

            sprite[1].position[0] = self.sprite_list[num * 4 + 1].vertices[1].position[0];
            sprite[1].position[1] = self.sprite_list[num * 4 + 1].vertices[1].position[1];

            sprite[2].position[0] = self.sprite_list[num * 4 + 2].vertices[2].position[0];
            sprite[2].position[1] = self.sprite_list[num * 4 + 2].vertices[2].position[1];

            sprite[3].position[0] = self.sprite_list[num * 4 + 3].vertices[3].position[0];
            sprite[3].position[1] = self.sprite_list[num * 4 + 3].vertices[3].position[1];
        }

        vb
    }

    fn get_index_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> Result<glium::IndexBuffer<u16>, glium::index::BufferCreationError> {
        let index_list = Vec::new();

        for (spi, s) in &self.sprite_list {
            let array_size = spi;
            for (iterator,i) in &s.indices {
                index_list.push(i + iterator * array_size);
            }
        }
        glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, index_list)
    }
}
