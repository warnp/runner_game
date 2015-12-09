use sprite::Sprite;
use vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;
use graphic_item::GraphicItem;

extern crate glium;
extern crate time;

use vertex;

// #[derive(Debug)]
pub struct SpriteManager<'a> {
    sprite_list: Rc<RefCell<Vec<Sprite<'a>>>>,
    display: &'a glium::backend::glutin_backend::GlutinFacade, /* vertex_buffer: glium::VertexBuffer<vertex::Vertex>,
                                                                * generation_id: i32, */
}

impl<'a> SpriteManager<'a> {
    #[warn(dead_code)]
    fn get_time() -> i32 {
        time::now().to_timespec().nsec
    }

    pub fn new(sprites: Vec<Sprite<'a>>,
               display: &'a glium::backend::glutin_backend::GlutinFacade)
               -> SpriteManager<'a> {

        SpriteManager {
            sprite_list: Rc::new(RefCell::new(sprites)),
            display: display,
        }

    }



    pub fn set_buffers(&self) -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {

        let vertices_array = self.sprite_list_to_vertex_list();
        let index_list = self.sprite_list_to_indices_buffer();


        (glium::VertexBuffer::dynamic(self.display, &vertices_array).unwrap(),
         glium::index::IndexBuffer::new(self.display,
                                        glium::index::PrimitiveType::TrianglesList,
                                        &index_list)
             .unwrap())
    }

    pub fn add_sprite(&mut self,
                      sprite: Sprite<'a>)
                      -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {

        self.sprite_list.borrow_mut().push(sprite);

        self.set_buffers()

    }


    pub fn delete_sprite(&mut self,
                         sprite_name: &str)
                         -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {


        self.sprite_list.borrow_mut().retain(|&x| x.name != sprite_name);

        self.set_buffers()

    }


    pub fn move_sprite(&self,
                       name: &str,
                       new_x: f32,
                       new_y: f32)
                       -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {
        let mut tmp = self.sprite_list.borrow_mut().clone();

        let mut res = tmp.iter_mut()
                         .enumerate()
                         .find(|x| (x.1).name == name);

        if res.is_some() {
            let mut sp = res.unwrap();

            (sp.1).vertices[0].position[0] = (sp.1).vertices[0].position[0] + new_x;
            (sp.1).vertices[1].position[0] = (sp.1).vertices[1].position[0] + new_x;
            (sp.1).vertices[2].position[0] = (sp.1).vertices[2].position[0] + new_x;
            (sp.1).vertices[3].position[0] = (sp.1).vertices[3].position[0] + new_x;

            (sp.1).vertices[0].position[1] = (sp.1).vertices[0].position[1] + new_y;
            (sp.1).vertices[1].position[1] = (sp.1).vertices[1].position[1] + new_y;
            (sp.1).vertices[2].position[1] = (sp.1).vertices[2].position[1] + new_y;
            (sp.1).vertices[3].position[1] = (sp.1).vertices[3].position[1] + new_y;

            self.sprite_list.borrow_mut()[sp.0] = *sp.1;
        }

        self.set_buffers()

    }

    pub fn get_sprite_list(&self) -> Vec<Sprite> {
        self.sprite_list.borrow().clone()
    }

    pub fn set_sprite_position
                               (&self,
                                name: &str,
                                new_x: f32,
                                new_y: f32)
                                -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {

        let mut tmp = self.sprite_list.borrow_mut().clone();

        let mut sp = tmp.iter_mut()
                        .enumerate()
                        .find(|x| (x.1).name == name)
                        .unwrap();


        (sp.1).vertices[0].position[0] = new_x;
        (sp.1).vertices[1].position[0] = new_x;
        (sp.1).vertices[2].position[0] = new_x;
        (sp.1).vertices[3].position[0] = new_x;

        (sp.1).vertices[0].position[1] = new_y;
        (sp.1).vertices[1].position[1] = new_y;
        (sp.1).vertices[2].position[1] = new_y;
        (sp.1).vertices[3].position[1] = new_y;

        self.sprite_list.borrow_mut()[sp.0] = *sp.1;

        self.set_buffers()

    }

    pub fn get_sprite(&self, name: &str) -> Sprite {
        let tmp = self.sprite_list.borrow().clone();
        let sp = tmp.iter()
                    .enumerate()
                    .find(|x| (x.1).name == name)
                    .unwrap();
        sp.1.clone()
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
    fn sprite_list_to_vertex_list(&self) -> Vec<Vertex> {
        let mut vertices_array: Vec<Vertex> = Vec::new();
        // println!("{:?}", self.sprite_list.into_inner());
        for sprite in &*self.sprite_list.borrow_mut() {

            vertices_array.push(sprite.vertices[0]);
            vertices_array.push(sprite.vertices[1]);
            vertices_array.push(sprite.vertices[2]);
            vertices_array.push(sprite.vertices[3]);

        }

        vertices_array
    }

    fn sprite_list_to_indices_buffer(&self) -> Vec<u16> {
        let mut index_list = Vec::with_capacity(self.sprite_list.borrow_mut().len() * 6);
        let mut iterator: u16 = 0;
        for s in &*self.sprite_list.borrow_mut() {
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
}

#[cfg(test)]
mod tests {

    use super::*;
    use sprite::Sprite;
    use glium::backend::Facade;
    use glium::{DisplayBuild, Surface};

    extern crate glium;


    #[test]
    fn should_set_vertex_buffer() {
        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("sprite",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);

        let mut vb = sprite_manager.set_buffers();

        // println!("TOTO ================   {:?}", vb.get_size());
        // println!("{:?}", vb.0.len());
        assert!(vb.0.len() > 0);
    }

    #[test]
    fn should_add_sprite() {

        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);

        let vertex_buffer = sprite_manager.set_buffers();

        let buffers = sprite_manager.add_sprite(Sprite::new("titi",
                                                            0.50,
                                                            0.50,
                                                            [1.0, 0.0, 0.0, 1.0],
                                                            0,
                                                            (1.0, 1.0)));

        assert!(buffers.0.len() == vertex_buffer.0.len() + 4);
        assert!(buffers.1.len() == vertex_buffer.1.len() + 6);
    }


    #[test]
    fn should_delete_sprite() {
        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);


        let buffers = sprite_manager.delete_sprite("toto");

        assert!(buffers.0.len() == 0);
        assert!(buffers.1.len() == 0);
    }

    #[test]
    fn should_move_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);
        let lst = sprite_manager.get_sprite_list();
        println!("first {:?}", lst);

        let sp = sprite_manager.move_sprite("toto", 1.0, 0.0);
        let lst = sprite_manager.get_sprite_list();
        println!("second {:?}", lst);
        assert!(sprite_manager.get_sprite_list()[0].vertices[0].position[0] == 0.0);
    }

    #[test]
    fn should_not_find_sprite_and_dont_move_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);

        let sp = sprite_manager.move_sprite("titi", 1.0, 0.0);
        let lst = sprite_manager.get_sprite_list();
        println!("second {:?}", lst);

        assert!(sprite_manager.get_sprite_list()[0].vertices[0].position[0] == -1.0);
    }

    #[ignore]
    #[test]
    fn should_return_sprite_vertices_coordinates() {
        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);
        // let sp = sprite_manager.get_sprites_coordinate("toto");

        // assert!(sp.0 == (-0.1,0.1));
    }

    #[test]
    fn should_get_sprite_list() {
        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);
        let lst = sprite_manager.get_sprite_list();
        assert_eq!(lst.len(), 1);

    }

    #[test]
    fn should_get_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto",
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0))],
                                                    &display);

        let sp = sprite_manager.get_sprite("toto");
        assert!(sp.name == "toto");
    }


}
