use sprite::Sprite;
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

        //TODO replace with a vector
        let mut vertices_array : Vec<sprite::Sprite> = Vec::new();

        for sprite in &self.sprite_list {

            let vertex = vertex::Vertex {
                position
            }
            vertices_array 0].position[0] = sprite.vertices[0].position[0];
            vertices_array 0].position[1] = sprite.vertices[0].position[1];
            vertices_array 0].tex_coords[0] = sprite.vertices[0].tex_coords[0];
            vertices_array 0].tex_coords[1] = sprite.vertices[0].tex_coords[1];
            vertices_array 0].i_tex_id = sprite.vertices[0].i_tex_id;

            vertices_array 1].position[0] = sprite.vertices[1].position[0];
            vertices_array 1].position[1] = sprite.vertices[1].position[1];
            vertices_array 1].tex_coords[0] = sprite.vertices[1].tex_coords[0];
            vertices_array 1].tex_coords[1] = sprite.vertices[1].tex_coords[1];
            vertices_array 1].i_tex_id = sprite.vertices[1].i_tex_id;

            vertices_array 2].position[0] = sprite.vertices[2].position[0];
            vertices_array 2].position[1] = sprite.vertices[2].position[1];
            vertices_array 2].tex_coords[0] = sprite.vertices[2].tex_coords[0];
            vertices_array 2].tex_coords[1] = sprite.vertices[2].tex_coords[1];
            vertices_array 2].i_tex_id = sprite.vertices[2].i_tex_id;

            vertices_array 3].position[0] = sprite.vertices[3].position[0];
            vertices_array 3].position[1] = sprite.vertices[3].position[1];
            vertices_array 3].tex_coords[0] = sprite.vertices[3].tex_coords[0];
            vertices_array 3].tex_coords[1] = sprite.vertices[3].tex_coords[1];
            vertices_array 3].i_tex_id = sprite.vertices[3].i_tex_id;
        }

        glium::VertexBuffer::dynamic(display, &vertices_array).unwrap();
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

    //TODO implement add and delete vertex safe sprite functions
    pub fn add_sprite(&self, sprite: Sprite) -> bool {
        return true;
    }

    pub fn delete_sprite(&self, sprite: Sprite) -> bool {
        return true;

    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use sprite::Sprite;
    use glium::backend::Facade;
    use glium::{DisplayBuild, Surface};

    extern crate glium;

    #[test]
    fn should_add_sprite() {

        let display = glium::glutin::WindowBuilder::new()
                                    .build_glium()
                                    .unwrap();

        let sprite_manager = SpriteManager::new(vec![Sprite::new(0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);

        let vertex_buffer = sprite_manager.get_vertex_buffer(&display);
        let indices = sprite_manager.get_index_buffer(&display).unwrap();

        let buffers = sprite_manager.add_sprite(Sprite::new(0.50,0.50,[1.0,0.0,0.0,1.0],0,(1.0,1.0)));

        assert!(buffers.0.len() == vertex_buffer.len()+4);
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
