use sprite::Sprite;
use vertex::Vertex;
use graphic_item::GraphicItem;

extern crate glium;

use vertex;

#[derive(Debug)]
pub struct SpriteManager {
    sprite_list: Vec<Sprite>,
    // vertex_buffer: glium::VertexBuffer<vertex::Vertex>,
}

impl SpriteManager {
    pub fn new(sprites: Vec<Sprite>) -> SpriteManager {
        SpriteManager{
            sprite_list: sprites,
        }

    }

    pub fn get_vertex_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> glium::VertexBuffer<vertex::Vertex>{

        let mut vertices_array : Vec<Vertex> = Vec::new();

        for sprite in &self.sprite_list {

            vertices_array.push(sprite.vertices[0]);
            vertices_array.push(sprite.vertices[1]);
            vertices_array.push(sprite.vertices[2]);
            vertices_array.push(sprite.vertices[3]);

        }

        glium::VertexBuffer::dynamic(display, &vertices_array).unwrap()
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

    pub fn add_sprite(&self, sprite: Sprite, vertex_buffer: glium::VertexBuffer<vertex::Vertex>) -> bool {
        self.sprite_list.push(sprite);
        {
            let mut mapping = vertex_buffer.map();
            //use slice(1..2).unwrap() instead, see documentation tests/buffer.rs
        }
    }

    pub fn delete_sprite(&self, sprite: Sprite, vertex_buffer: glium::VertexBuffer<vertex::Vertex>) -> (bool) {
        (true)

    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use sprite::Sprite;
    use glium::backend::Facade;
    use glium::{DisplayBuild, Surface};

    extern crate glium;

    #[ignore]
    #[test]
    fn should_set_vertex_buffer(){
        let display = glium::glutin::WindowBuilder::new()
                                    .build_glium()
                                    .unwrap();

        let sprite_manager = SpriteManager::new(vec![Sprite::new(0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);

        let vb = sprite_manager.get_vertex_buffer(&display);

        // println!("TOTO ================   {:?}", vb.get_size());
        assert_eq!(vb.map().len(),1);
    }

    #[test]
    fn should_add_sprite() {

        let display = glium::glutin::WindowBuilder::new()
                                    .build_glium()
                                    .unwrap();

        let sprite_manager = SpriteManager::new(vec![Sprite::new(0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);

        let vertex_buffer = sprite_manager.get_vertex_buffer(&display);
        let indices = sprite_manager.get_index_buffer(&display).unwrap();

        let buffers = sprite_manager.add_sprite(Sprite::new(0.50,0.50,[1.0,0.0,0.0,1.0],0,(1.0,1.0)));

        // assert!(buffers.0.len() == vertex_buffer.len()+4);
    }

    #[ignore]
    #[test]
    #[should_panic]
    fn should_not_add_sprite() {

    }

    #[ignore]
    #[test]
    fn should_delete_sprite() {

    }

    #[ignore]
    #[test]
    #[should_panic]
    fn should_not_delete_sprite() {

    }
}
