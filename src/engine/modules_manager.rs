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

pub struct ModulesManager;

impl ModulesManager {
    pub fn start() {

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
            // thread::sleep_ms(200);
            let fps = engine_helper.get_fps();
            println!("{}", fps.0);
            let time = engine_helper.get_iterator();
            // println!("{}", time);
            GraphicsHandler::draw(&display, sprite_manager.set_buffers(), &textures, &program);
            let mut result = "";


            InputManager::get_input(&display);
            // if result != "" {
            //     println!("{:?}", result);
            //     input_buffer.push(result);
            //     // input_buffer.push(InputManager::get_input(&display));
            //
            // }

            // println!("{:#?}", input_buffer);
            // if input_buffer.contains(&"escape_press") {
            //     return;
            // }
        }

    }
}
