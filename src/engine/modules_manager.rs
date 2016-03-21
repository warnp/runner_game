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
use std::boxed::Box;

pub struct ModulesManager{
    pub generics: Vec<Box<GenericObject>>,
}

impl ModulesManager {

    pub fn new() -> ModulesManager {
        let tmp_gen :Vec<Box<GenericObject>> = Vec::new();
        ModulesManager{
            generics: tmp_gen,
        }
    }

    pub fn new_with_generics(generics: Vec<Box<GenericObject>>) -> ModulesManager {
        ModulesManager{
            generics: generics,
        }
    }
    pub fn start(&self) {

        // ---------DISPLAY--------------
        let display = glium::glutin::WindowBuilder::new()
                          .with_vsync()
                          .with_dimensions(1024, 768)
                          .build_glium()
                          .unwrap();

        let mut engine_helper = EngineHelper::new();

        // GraphicsHandler::compile_shaders(&display, vec![], "simple_shader");

        let mut shaders = Shaders::new(vec![&include_bytes!("../../content/VFKM2.png")[..]]);
        shaders.compile_shaders(&display);
        let program = shaders.get_compiled_shader("simple_shader");
        let textures = shaders.get_texture_array(&display);

        let mut sprite_manager = SpriteManager::new(vec![], &display);
        let mut buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>);
        buffers = sprite_manager.set_buffers();

        // ---------INPUT------------------
        let mut input_buffer = vec![""];
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


        loop {

            let fps = engine_helper.get_fps();
            // println!("{}", fps.0);
            let time = engine_helper.get_iterator();

            GraphicsHandler::draw(&display, sprite_manager.set_buffers(), &textures, &program);
            let mut result = "";


            input_buffer = InputManager::get_input(&display);

            for el in &input_buffer {
                if el.to_string() == "d_press".to_string() {
                        println!("{:#?}", input_buffer );
                }
            }

        }

    }
}
