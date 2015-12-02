use sprite::Sprite;
use vertex::Vertex;
use graphic_item::GraphicItem;

extern crate glium;
extern crate time;

use vertex;

#[derive(Debug, Clone)]
pub struct SpriteManager<'a> {
    sprite_list: Vec<Sprite<'a>>, /* vertex_buffer: glium::VertexBuffer<vertex::Vertex>,
                                   * generation_id: i32, */
}

impl<'a> SpriteManager<'a> {
    #[warn(dead_code)]
    fn get_time() -> i32 {
        time::now().to_timespec().nsec
    }

    pub fn new(sprites: Vec<Sprite>) -> SpriteManager {
        SpriteManager { sprite_list: sprites }

    }



    pub fn set_buffers(&mut self,
                       display: &glium::backend::glutin_backend::GlutinFacade)
                       -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {

        let vertices_array = self.sprite_list_to_vertex_list();
        let index_list = self.sprite_list_to_indices_buffer();


        (glium::VertexBuffer::dynamic(display, &vertices_array).unwrap(),
         glium::index::IndexBuffer::new(display,
                                        glium::index::PrimitiveType::TrianglesList,
                                        &index_list)
             .unwrap())
    }

    pub fn add_sprite(&mut self, sprite: Sprite<'a>) -> SpriteManager {

        self.sprite_list.push(sprite);

        SpriteManager { sprite_list: self.sprite_list.clone() }

    }


    pub fn delete_sprite(&mut self,
                         sprite_name: &str,
                         display: &glium::backend::glutin_backend::GlutinFacade)
                         -> SpriteManager {


        self.sprite_list.retain(|&x| x.name != sprite_name);

        SpriteManager { sprite_list: self.sprite_list.clone() }

    }


    pub fn move_sprite(&mut self, name: &str, new_x: f32, new_y: f32) -> SpriteManager {
        let mut tmp = self.sprite_list.clone();
        let mut sp = self.sprite_list.iter_mut().enumerate().find(|x| (x.1).name != name).unwrap();
        (sp.1).vertices[0].position[0] = (sp.1).vertices[0].position[0] + new_x;
        (sp.1).vertices[1].position[0] = (sp.1).vertices[1].position[0] + new_x;
        (sp.1).vertices[2].position[0] = (sp.1).vertices[2].position[0] + new_x;
        (sp.1).vertices[3].position[0] = (sp.1).vertices[3].position[0] + new_x;

        (sp.1).vertices[0].position[1] = (sp.1).vertices[0].position[1] + new_y;
        (sp.1).vertices[1].position[1] = (sp.1).vertices[1].position[1] + new_y;
        (sp.1).vertices[2].position[1] = (sp.1).vertices[2].position[1] + new_y;
        (sp.1).vertices[3].position[1] = (sp.1).vertices[3].position[1] + new_y;

        tmp[sp.0] = *sp.1;

        SpriteManager { sprite_list: tmp }


    }

    // pub fn get_sprites_coordinate(&self, name: &str) -> ((f32,f32),(f32,f32),(f32,f32),(f32,f32)){
    //     let sp = self.get_sprite(name);
    //
    //     ((sp.1.vertices[0].position[0],sp.1.vertices[0].position[1]),
    //     (sp.1.vertices[1].position[0],sp.1.vertices[1].position[1]),
    //     (sp.1.vertices[2].position[0],sp.1.vertices[2].position[1]),
    //     (sp.1.vertices[3].position[0],sp.1.vertices[3].position[1]))
    //
    // }
    //
    // fn get_sprite(&self, name: &str) -> (usize, Sprite) {
    //     let mut tmp = self.get_temp_sprite_list();
    //
    //     let res = tmp.iter().enumerate().find(|&x| x.1.name == name).unwrap();
    //     (res.0, *res.1)
    // }
    //
    fn sprite_list_to_vertex_list(&mut self) -> Vec<Vertex> {
        let mut vertices_array: Vec<Vertex> = Vec::new();
        // println!("{:?}", self.sprite_list.into_inner());
        for sprite in &self.sprite_list {

            vertices_array.push(sprite.vertices[0]);
            vertices_array.push(sprite.vertices[1]);
            vertices_array.push(sprite.vertices[2]);
            vertices_array.push(sprite.vertices[3]);

        }

        vertices_array
    }

    fn sprite_list_to_indices_buffer(&mut self) -> Vec<u16> {
        let mut index_list = Vec::with_capacity(self.sprite_list.len() * 6);
        let mut iterator: u16 = 0;
        for s in &self.sprite_list {
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
    //
    // fn return_vertex_and_index_lists(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> (glium::VertexBuffer<vertex::Vertex>,glium::IndexBuffer<u16>){
    //     let vertices_array = self.sprite_list_to_vertex_list();
    //     let index_list = self.sprite_list_to_indices_buffer();
    //     (glium::VertexBuffer::dynamic(display, &vertices_array).unwrap(), glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &index_list).unwrap())
    // }
    //
    // fn get_temp_sprite_list(&mut self) -> Vec<Sprite<'a>> {
    //     let mut tmp = Vec::new();
    //     tmp.extend((*self.sprite_list.borrow_mut()).iter().cloned());
    //
    //     tmp
    // }
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
    fn should_set_vertex_buffer() {
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

        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                 0.0,
                                                                 0.0,
                                                                 [1.0, 0.0, 0.0, 1.0],
                                                                 0,
                                                                 (1.0, 1.0))]);

        // let vertex_buffer = sprite_manager.get_vertex_buffer(&display);
        // let index_buffer = sprite_manager.get_index_buffer(&display);

        // let buffers = sprite_manager.add_sprite(Sprite::new("titi",0.50,0.50,[1.0,0.0,0.0,1.0],0,(1.0,1.0)), &display);

        // assert!(buffers.0.len() == vertex_buffer.len()+4);
        // assert!(buffers.1.len() == index_buffer.len()+6);
    }


    #[test]
    fn should_delete_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                 0.0,
                                                                 0.0,
                                                                 [1.0, 0.0, 0.0, 1.0],
                                                                 0,
                                                                 (1.0, 1.0))]);

        // let vertex_buffer = sprite_manager.get_vertex_buffer(&display);
        // let index_buffer = sprite_manager.get_index_buffer(&display);

        // let buffers = sprite_manager.delete_sprite("toto", &display);

        // assert!(buffers.0.len() == 0);
        // assert!(buffers.1.len() == 0);
    }

    #[test]
    fn should_move_sprite() {
        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                 0.0,
                                                                 0.0,
                                                                 [1.0, 0.0, 0.0, 1.0],
                                                                 0,
                                                                 (1.0, 1.0))]);
        // let sp = sprite_manager.move_sprite("toto", 1.0, 0.0);

        // assert!(sp.vertices[0].position[0] == 0.9);
    }

    #[test]
    fn should_return_sprite_vertices_coordinates() {
        let sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                 0.0,
                                                                 0.0,
                                                                 [1.0, 0.0, 0.0, 1.0],
                                                                 0,
                                                                 (1.0, 1.0))]);
        // let sp = sprite_manager.get_sprites_coordinate("toto");

        // assert!(sp.0 == (-0.1,0.1));
    }


}
