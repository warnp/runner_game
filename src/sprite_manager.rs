use sprite::{Sprite, GraphicItem};


extern crate glium;

use vertex;

#[derive(Debug)]
pub struct SpriteManager {
    sprite_list: Vec<Sprite>,
}

impl SpriteManager {
    pub fn new(sprites: Vec<Sprite>) -> SpriteManager {
        SpriteManager{
            sprite_list: sprites,
        }

    }

    pub fn get_vertex_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> glium::VertexBuffer<vertex::Vertex>{

        let mut vb : glium::VertexBuffer<vertex::Vertex> = glium::VertexBuffer::empty_dynamic(display, self.sprite_list.len() * 4).unwrap();

        for (num, sprite) in vb.map().chunks_mut(4).enumerate() {

            sprite[0].position[0] = self.sprite_list[num].vertices[0].position[0];
            sprite[0].position[1] = self.sprite_list[num].vertices[0].position[1];
            sprite[0].tex_coords[0] = self.sprite_list[num].vertices[0].tex_coords[0];
            sprite[0].tex_coords[1] = self.sprite_list[num].vertices[0].tex_coords[1];
            sprite[0].i_tex_id = self.sprite_list[num].vertices[0].i_tex_id;

            sprite[1].position[0] = self.sprite_list[num].vertices[1].position[0];
            sprite[1].position[1] = self.sprite_list[num].vertices[1].position[1];
            sprite[1].tex_coords[0] = self.sprite_list[num].vertices[1].tex_coords[0];
            sprite[1].tex_coords[1] = self.sprite_list[num].vertices[1].tex_coords[1];
            sprite[1].i_tex_id = self.sprite_list[num].vertices[1].i_tex_id;

            sprite[2].position[0] = self.sprite_list[num].vertices[2].position[0];
            sprite[2].position[1] = self.sprite_list[num].vertices[2].position[1];
            sprite[2].tex_coords[0] = self.sprite_list[num].vertices[2].tex_coords[0];
            sprite[2].tex_coords[1] = self.sprite_list[num].vertices[2].tex_coords[1];
            sprite[2].i_tex_id = self.sprite_list[num].vertices[2].i_tex_id;

            sprite[3].position[0] = self.sprite_list[num].vertices[3].position[0];
            sprite[3].position[1] = self.sprite_list[num].vertices[3].position[1];
            sprite[3].tex_coords[0] = self.sprite_list[num].vertices[3].tex_coords[0];
            sprite[3].tex_coords[1] = self.sprite_list[num].vertices[3].tex_coords[1];
            sprite[3].i_tex_id = self.sprite_list[num].vertices[3].i_tex_id;
        }

        vb
    }

    pub fn get_index_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> Result<glium::IndexBuffer<u16>, glium::index::BufferCreationError> {
        let mut index_list = Vec::with_capacity(self.sprite_list.len() * 6);
        let mut iterator : u16 = 0;
        for s in &self.sprite_list {
            index_list.push(s.indices[0] + 4 * iterator);
            index_list.push(s.indices[1] + 4 * iterator);
            index_list.push(s.indices[2] + 4 * iterator);
            index_list.push(s.indices[3] + 4 * iterator);
            index_list.push(s.indices[4] + 4 * iterator);
            index_list.push(s.indices[5] + 4 * iterator);




            iterator = iterator + 1;

        }

        glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index_list)
    }
}
