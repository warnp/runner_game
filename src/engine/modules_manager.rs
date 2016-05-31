extern crate glium;
use engine::sprite::Sprite;
use engine::shader_manager::{Shaders, ShaderCouple};
use engine::sprite_manager::SpriteManager;
use engine::graphics_handler::GraphicsHandler;
use glium::{VertexBuffer, IndexBuffer, DisplayBuild};
use engine::vertex::Vertex;
use engine::input_manager::InputManager;
use std::sync::mpsc;
use std::thread;
use engine::engine_helper::EngineHelper;
use engine::generic_object::GenericObject;
use engine::generic_control::GenericControl;
use std::cell::RefCell;
use std::borrow::Cow;
use engine::text_writer::TextWriter;
// use std::boxed::Box;

pub struct ModulesManager{
    display: glium::backend::glutin_backend::GlutinFacade,
    program: glium::program::Program,
    textures: glium::texture::Texture2dArray,
    // sprite_manager: SpriteManager<'a>,

}

impl ModulesManager {

    pub fn new() -> ModulesManager{
        let display = glium::glutin::WindowBuilder::new()
                          .with_vsync()
                          .with_dimensions(800, 600)
                          .build_glium()
                          .unwrap();

          let mut shaders = Shaders::new(vec![&include_bytes!("../../content/VFKM2.png")[..],
                                                              &include_bytes!("../../content/11532.png")[..],
                                                              &include_bytes!("../../content/NatureForests.png")[..],
                                                              &include_bytes!("../../content/hero.png")[..]], &display);
          shaders.compile_shaders(&display);
          let textures = shaders.get_texture_array(&display);
            ModulesManager{
                display: display,
                program: shaders.get_compiled_shader("simple_shader"),
                textures: textures,
                // sprite_manager: buffers,
            }
    }



    pub fn draw(&mut self,
         delta_time: f64,
         generics_objects: &Vec<Box<GenericObject>>,
         generics_controls: Vec<Box<GenericControl>>) -> &ModulesManager {

        let bunch_of_generic_objects = self.generic_object_interpretor(generics_objects).get_buffers(&self.display);


         GraphicsHandler::draw(&self.display,
              bunch_of_generic_objects,
              &self.textures,
              &self.program);
        self
    }

    //TODO Object identification by string is not cool
    pub fn generic_object_interpretor(&self, generic_object:  &Vec<Box<GenericObject>>) -> SpriteManager{
        let mut result_vec = Vec::new();
        let mut name : String;
        let mut position: (f32,f32,f32);
        let mut description : String;
        for i in generic_object {
            name = i.get_name();
            position = i.get_position();
            description = i.get_description();
            match i.get_type().as_ref() {
                "Sprite" => {result_vec.push(Sprite::new(name, position.0,position.1,[1.0,0.0,0.0,1.0],1,(0.1,0.1),0));},

                "Text" => {
                        let text_writer = TextWriter::new(0,(256,256),(16,16),0.05,(position.0,position.1),&name, true);
                        result_vec.extend_from_slice(&text_writer.get_string(description.as_str()));
                    },
                _ => ()
            }

        }
        SpriteManager::new(result_vec)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use engine::generic_object::GenericObject;

#[derive(Debug)]
struct ObjTest {
    size: i32,
}
impl GenericObject for ObjTest {
    fn get_type(&self) -> String {
        "Sprite".to_string()
    }

    fn get_position(&self) -> (f32,f32,f32){
        (0.0,0.0,0.0)
    }
    fn get_name(&self) -> String {
        "Test".to_string()
    }
    fn get_description(&self) -> String {
        "This is a test description".to_string()
    }
}

    #[test]
    fn should_return_modules_manager(){
        let mut modules_manager = ModulesManager::new();

        let new_mod = modules_manager.draw(5.0, &vec![], vec![]);

//TODO need to find a way to test that
        // match new_mod {
        //     Some(x) => assert!(true),
        //     None => assert!(false)
        // }
        // assert!(&modules_manager == &new_mod);
        assert!(false);
    }

    #[test]
    fn should_interpret_generic_object(){
        let modules_manager = ModulesManager::new();

        let object_list = modules_manager.generic_object_interpretor(&vec![Box::new(ObjTest{size: 1})]);
        assert!(object_list.get_sprite_list().len() == 1);
    }
}
