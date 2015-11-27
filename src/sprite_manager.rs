use sprite::Sprite;
use vertex::Vertex;
use graphic_item::GraphicItem;
use std::cell::RefCell;

extern crate glium;

use vertex;

#[derive(Debug)]
pub struct SpriteManager<'a> {
    pub sprite_list: RefCell<Vec<Sprite<'a>>>,
    // vertex_buffer: glium::VertexBuffer<vertex::Vertex>,
}

impl<'a> SpriteManager<'a> {


    pub fn new(sprites: Vec<Sprite>) -> SpriteManager{
        SpriteManager{
            sprite_list: RefCell::new(sprites),

        }

    }



    pub fn get_vertex_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> glium::VertexBuffer<vertex::Vertex>{

        let vertices_array = self.sprite_list_to_vertex_list();

        glium::VertexBuffer::dynamic(display, &vertices_array).unwrap()
    }

    pub fn get_index_buffer(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> glium::IndexBuffer<u16> {
        let index_list = self.sprite_list_to_indices_buffer();

        glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index_list).unwrap()
    }

    pub fn add_sprite(&self,sprite: Sprite<'a>, display: &glium::backend::glutin_backend::GlutinFacade) -> (glium::VertexBuffer<vertex::Vertex>,glium::IndexBuffer<u16>) {
        let mut tmp = self.get_temp_sprite_list();
        tmp.push(sprite);

        *self.sprite_list.borrow_mut() = tmp;
        self.return_vertex_and_index_lists(display)

    }

    pub fn delete_sprite(&self, sprite_name: &str, display: &glium::backend::glutin_backend::GlutinFacade) -> (glium::VertexBuffer<vertex::Vertex>,glium::IndexBuffer<u16>) {
        let mut tmp = self.get_temp_sprite_list();

        tmp.retain(|&x| x.name != sprite_name);
        *self.sprite_list.borrow_mut() = tmp;


        self.return_vertex_and_index_lists(display)

    }


    pub fn move_sprite(&self, name: &str, new_x: f32, new_y: f32) {
        let mut sp_full = self.get_sprite(name);
        // let mut sp = self.spriter;
        let mut sp = sp_full.1.clone();
        // println!("{:?}", sp[0].vertices[0].position[1]);
        sp.vertices[0].position[0] = sp.vertices[0].position[0] + new_x;
        sp.vertices[1].position[0] = sp.vertices[1].position[0] + new_x;
        sp.vertices[2].position[0] = sp.vertices[2].position[0] + new_x;
        sp.vertices[3].position[0] = sp.vertices[3].position[0] + new_x;

        sp.vertices[0].position[1] = sp.vertices[0].position[1] + new_y;
        sp.vertices[1].position[1] = sp.vertices[1].position[1] + new_y;
        sp.vertices[2].position[1] = sp.vertices[2].position[1] + new_y;
        sp.vertices[3].position[1] = sp.vertices[3].position[1] + new_y;

        // self.sprite_list.borrow_mut()[sp_full.0] = sp;

        // sp


    }

    pub fn get_sprites_coordinate(&self, name: &str) -> ((f32,f32),(f32,f32),(f32,f32),(f32,f32)){
        let sp = self.get_sprite(name);

        ((sp.1.vertices[0].position[0],sp.1.vertices[0].position[1]),
        (sp.1.vertices[1].position[0],sp.1.vertices[1].position[1]),
        (sp.1.vertices[2].position[0],sp.1.vertices[2].position[1]),
        (sp.1.vertices[3].position[0],sp.1.vertices[3].position[1]))

    }

    fn get_sprite(&self, name: &str) -> (usize, Sprite) {
        let mut tmp = self.get_temp_sprite_list();

        let res = tmp.iter().enumerate().find(|&x| x.1.name == name).unwrap();
        (res.0, *res.1)
    }

    fn sprite_list_to_vertex_list(&self) -> Vec<Vertex>{
        let mut vertices_array : Vec<Vertex> = Vec::new();
        // println!("{:?}", self.sprite_list.into_inner());
        for sprite in &*self.sprite_list.borrow() {

            vertices_array.push(sprite.vertices[0]);
            vertices_array.push(sprite.vertices[1]);
            vertices_array.push(sprite.vertices[2]);
            vertices_array.push(sprite.vertices[3]);

        }

        vertices_array
    }

    fn sprite_list_to_indices_buffer(&self) -> Vec<u16>{
        let mut index_list = Vec::with_capacity(self.sprite_list.borrow().len() * 6);
        let mut iterator : u16 = 0;
        for s in &*self.sprite_list.borrow() {
            index_list.push(s.indices[0] + 4 * iterator);
            index_list.push(s.indices[1] + 4 * iterator);
            index_list.push(s.indices[2] + 4 * iterator);
            index_list.push(s.indices[3] + 4 * iterator);
            index_list.push(s.indices[4] + 4 * iterator);
            index_list.push(s.indices[5] + 4 * iterator);

            iterator = iterator + 1;

        }

        index_list
    }

    fn return_vertex_and_index_lists(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> (glium::VertexBuffer<vertex::Vertex>,glium::IndexBuffer<u16>){
        let vertices_array = self.sprite_list_to_vertex_list();
        let index_list = self.sprite_list_to_indices_buffer();
        (glium::VertexBuffer::dynamic(display, &vertices_array).unwrap(), glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index_list).unwrap())
    }

    fn get_temp_sprite_list(&mut self) -> Vec<Sprite<'a>> {
        let mut tmp = Vec::new();
        tmp.extend((*self.sprite_list.borrow_mut()).iter().cloned());

        tmp
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

        // let sprite_manager = SpriteManager::new(vec![Sprite::new(0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);

        // let vb = sprite_manager.get_vertex_buffer(&display);

        // println!("TOTO ================   {:?}", vb.get_size());
        // assert_eq!(vb.map().len(),1);
    }

    #[test]
    fn should_add_sprite() {

        let display = glium::glutin::WindowBuilder::new()
                                    .build_glium()
                                    .unwrap();

        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);

        let vertex_buffer = sprite_manager.get_vertex_buffer(&display);
        let index_buffer = sprite_manager.get_index_buffer(&display);

        let buffers = sprite_manager.add_sprite(Sprite::new("titi",0.50,0.50,[1.0,0.0,0.0,1.0],0,(1.0,1.0)), &display);

        assert!(buffers.0.len() == vertex_buffer.len()+4);
        assert!(buffers.1.len() == index_buffer.len()+6);
    }


    #[test]
    fn should_delete_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                                    .build_glium()
                                    .unwrap();

        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);

        let vertex_buffer = sprite_manager.get_vertex_buffer(&display);
        let index_buffer = sprite_manager.get_index_buffer(&display);

        let buffers = sprite_manager.delete_sprite("toto", &display);

        assert!(buffers.0.len() == 0);
        assert!(buffers.1.len() == 0);
    }

    #[test]
    fn should_move_sprite(){
        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);
        let sp = sprite_manager.move_sprite("toto",1.0,0.0);

        // assert!(sp.vertices[0].position[0] == 0.9);
    }

    #[test]
    fn should_return_sprite_vertices_coordinates(){
        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",0.0,0.0,[1.0,0.0,0.0,1.0],0,(1.0,1.0))]);
        let sp = sprite_manager.get_sprites_coordinate("toto");

        assert!(sp.0 == (-0.1,0.1));
    }


}
