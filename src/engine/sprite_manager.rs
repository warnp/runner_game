use engine::sprite::Sprite;
use engine::vertex::Vertex;
use std::cell::RefCell;
use std::rc::Rc;
use engine::vertex;

extern crate glium;
extern crate time;


/// The `SpriteManager` type.
// #[derive(Debug)]
pub struct SpriteManager {
    /// Sprite list to handle with
    sprite_list: Rc<RefCell<Vec<Sprite>>>,
}

impl SpriteManager {

    /// Construct new `SpriteManager`
    ///
    /// # Examples
    ///
    /// ```
    /// use sprite_manager::SpriteManager;
    ///
    /// let manager = SpriteManager::new(vec![Sprite("toto", 0.0,0.0,[0.0,0.0,0.0,0.0],1,(1.0,1.0),0)], &display);
    /// ```
    pub fn new(sprites: Vec<Sprite>)
               -> SpriteManager {
        SpriteManager {
            sprite_list: Rc::new(RefCell::new(sprites)),
        }

    }



    pub fn get_buffers(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {

        let vertices_array = self.sprite_list_to_vertex_list();
        let index_list = self.sprite_list_to_indices_buffer();

        (glium::VertexBuffer::dynamic(display, &vertices_array).unwrap(),
         glium::index::IndexBuffer::new(display,
                                        glium::index::PrimitiveType::TrianglesList,
                                        &index_list)
             .unwrap())
    }

    pub fn add_sprite(&mut self,
                      sprite: Sprite,display: &glium::backend::glutin_backend::GlutinFacade)
                      -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {

        self.sprite_list.borrow_mut().push(sprite);

        self.get_buffers(display)

    }


    pub fn delete_sprite(&mut self,
                         sprite_name: &str, display: &glium::backend::glutin_backend::GlutinFacade)
                         -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {


        self.sprite_list.borrow_mut().retain(|x| x.name != sprite_name);

        self.get_buffers(display)

    }


    pub fn move_sprite(&self,
                       name: &str,
                       new_x: f32,
                       new_y: f32, display: &glium::backend::glutin_backend::GlutinFacade)
                       -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {
        let mut tmp = self.sprite_list.borrow_mut().clone();

        let res = tmp.iter_mut()
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

        }

        self.get_buffers(display)

    }

    pub fn get_sprite_list(&self) -> Vec<Sprite> {
        self.sprite_list.borrow().clone()
    }

    pub fn set_sprite_position
                               (&self,
                                name: &str,
                                new_x: f32,
                                new_y: f32, display: &glium::backend::glutin_backend::GlutinFacade)
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

        self.get_buffers(&display)

    }

    pub fn get_sprite(&self, name: &str) -> Sprite {
        let tmp = self.sprite_list.borrow().clone();
        let sp = tmp.iter()
                    .enumerate()
                    .find(|x| (x.1).name == name)
                    .unwrap();
        sp.1.clone()
    }

    pub fn order_sprites(&self) {
        self.sprite_list.borrow_mut().sort_by(|a, b| a.cmp(b));
    }

    fn set_uv(&self,
              name: &str,
              new_coordinates: [f32; 2],
              uv_size: (f32, f32), display: &glium::backend::glutin_backend::GlutinFacade)
              -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {
        let mut tmp = self.sprite_list.borrow_mut().clone();

        let mut sp = tmp.iter_mut()
                        .enumerate()
                        .find(|x| (x.1).name == name)
                        .unwrap();


        (sp.1).vertices[0].tex_coords[0] = new_coordinates[0];
        (sp.1).vertices[1].tex_coords[0] = new_coordinates[0] + uv_size.0;
        (sp.1).vertices[2].tex_coords[0] = new_coordinates[0] + uv_size.0;
        (sp.1).vertices[3].tex_coords[0] = new_coordinates[0];

        (sp.1).vertices[0].tex_coords[1] = new_coordinates[1];
        (sp.1).vertices[1].tex_coords[1] = new_coordinates[1];
        (sp.1).vertices[2].tex_coords[1] = new_coordinates[1] + uv_size.1;
        (sp.1).vertices[3].tex_coords[1] = new_coordinates[1] + uv_size.1;

        self.get_buffers(display)
    }


    fn sprite_list_to_vertex_list(&self) -> Vec<Vertex> {
        let mut vertices_array: Vec<Vertex> = Vec::new();
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
    use engine::sprite::Sprite;
    use glium::backend::Facade;
    use glium::{DisplayBuild, Surface};

    extern crate glium;

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_set_vertex_buffer() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("sprite".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);

        let mut vb = sprite_manager.get_buffers(&display);


        assert!(vb.0.len() > 0);
    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_add_sprite() {

        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);

        let vertex_buffer = sprite_manager.get_buffers(&display);

        let buffers = sprite_manager.add_sprite(Sprite::new("titi".to_string(),
                                                            0.50,
                                                            0.50,
                                                            [1.0, 0.0, 0.0, 1.0],
                                                            0,
                                                            (1.0, 1.0),
                                                            1), &display);

        assert!(buffers.0.len() == vertex_buffer.0.len() + 4);
        assert!(buffers.1.len() == vertex_buffer.1.len() + 6);
    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_delete_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();
        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);


        let buffers = sprite_manager.delete_sprite("toto", &display);

        assert!(buffers.0.len() == 0);
        assert!(buffers.1.len() == 0);
    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_move_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);
        let lst = sprite_manager.get_sprite_list();
        println!("first {:?}", lst);

        let sp = sprite_manager.move_sprite("toto", 1.0, 0.0, &display);
        let lst = sprite_manager.get_sprite_list();
        println!("second {:?}", lst);
        assert!(sprite_manager.get_sprite_list()[0].vertices[0].position[0] == 0.5);
    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_not_find_sprite_and_dont_move_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);

        let sp = sprite_manager.move_sprite("titi", 1.0, 0.0, &display);
        let lst = sprite_manager.get_sprite_list();

        assert!(sprite_manager.get_sprite_list()[0].vertices[0].position[0] == -0.5);
    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_get_sprite_list() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);
        let lst = sprite_manager.get_sprite_list();
        assert_eq!(lst.len(), 1);

    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_get_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);

        let sp = sprite_manager.get_sprite("toto");
        assert!(sp.name == "toto");
    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_order_sprite() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     1),
                                                         Sprite::new("titi".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);

        sprite_manager.order_sprites();
        let mut index = 10;

        for (i, e) in sprite_manager.get_sprite_list().iter().enumerate() {
            if e.name == "titi" {
                index = i;
            }
        }

        println!("{}", index);
        assert!(index == 0);

    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_move_uv() {
        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

        let mut sprite_manager = SpriteManager::new(vec![Sprite::new("toto".to_string(),
                                                                     0.0,
                                                                     0.0,
                                                                     [1.0, 0.0, 0.0, 1.0],
                                                                     0,
                                                                     (1.0, 1.0),
                                                                     0)]);



        let sp = sprite_manager.set_uv("toto", [0.1, 0.1], (0.1, 0.1), &display);
        assert!(sprite_manager.get_sprite_list()[0].vertices[0].tex_coords[0] == 0.1);

    }


}
