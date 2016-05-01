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
// use std::boxed::Box;

pub struct ModulesManager<'a>{
    display: glium::backend::glutin_backend::GlutinFacade,
    shaders: Shaders<'a>,
    program: glium::program::Program,

}

impl<'a> ModulesManager<'a> {

    pub fn new() -> ModulesManager<'a>{
        let display = glium::glutin::WindowBuilder::new()
                          .with_vsync()
                          .with_dimensions(1024, 768)
                          .build_glium()
                          .unwrap();

          let mut shaders = Shaders::new(vec![&include_bytes!("../../content/VFKM2.png")[..]], &display);
          shaders.compile_shaders(&display);
            ModulesManager{
                display: display,
                program: shaders.get_compiled_shader("simple_shader"),
                shaders: shaders,
            }
    }

    //Should be a private method and should be used in ctors
    pub fn start(&self) {
        // &self.shaders.compile_shaders(&self.display);

        // ---------DISPLAY--------------
        // let display = glium::glutin::WindowBuilder::new()
        //                   .with_vsync()
        //                   .with_dimensions(1024, 768)
        //                   .build_glium()
        //                   .unwrap();

        // let mut engine_helper = EngineHelper::new();

        // GraphicsHandler::compile_shaders(&display, vec![], "simple_shader");

        // let mut shaders = Shaders::new(vec![&include_bytes!("../../content/VFKM2.png")[..]]);
        // shaders.compile_shaders(&display);
        // let program = shaders.get_compiled_shader("simple_shader");
        // let textures = shaders.get_texture_array(&display);

        let mut sprite_manager = SpriteManager::new(vec![], &self.display);
        let mut buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>);
        buffers = sprite_manager.get_buffers();

        // ---------INPUT------------------
        // let mut input_buffer = vec![""];
        // let (tx, rx): (mpsc::Sender<&glium::backend::glutin_backend::GlutinFacade>,
        //                mpsc::Receiver<&glium::backend::glutin_backend::GlutinFacade>) =
        //     mpsc::channel();

        // On peut garder ça pour faire du gpgpu plus tard
        // let graphics = thread::spawn(|| {
        //     let display = glium::glutin::WindowBuilder::new()
        //                       .with_visibility(false)
        //                       .build_glium()
        //                       .unwrap();
        //     loop {
        //         let key = InputManager::get_input(&display);
        //         // println!("{}", key);
        //     }
        // });


        // loop {
        //
        //     // let fps = engine_helper.get_fps();
        //     // println!("{}", fps.0);
        //     // let time = engine_helper.get_iterator();
        //
        //
        //     let mut result = "";
        //
        //
        //     input_buffer = InputManager::get_input(&display);
        //
        //     for el in &input_buffer {
        //         if el.to_string() == "d_press".to_string() {
        //                 println!("{:#?}", input_buffer );
        //         }
        //     }
        //
        // }

    }

    pub fn draw(&mut self,
         delta_time: f64,
         generics_objects: Vec<Box<GenericObject>>,
         generics_controls: Vec<Box<GenericControl>>) -> &ModulesManager {

             self.generic_object_interpretor(generics_objects);
         let buffers = SpriteManager::new(vec![], &self.display).get_buffers();
         GraphicsHandler::draw(&self.display,
              buffers,
              &self.shaders.get_texture_array(&self.display),
              &self.program);

        self
    }

    pub fn generic_object_interpretor(&self, generic_object: Vec<Box<GenericObject>>) -> Vec<Sprite>{
        for i in generic_object {
            println!("{:?}", i.size);
        }
        vec![]
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

}

    #[test]
    fn should_return_modules_manager(){
        let mut modules_manager = ModulesManager::new();

        let new_mod = modules_manager.draw(5.0, vec![], vec![]);

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

        let object_list = modules_manager.generic_object_interpretor(vec![Box::new(ObjTest{size: 1})]);
        assert!(object_list.len() == 1);
    }
}
