#[macro_use]
extern crate glium;
extern crate time;
extern crate rand;


mod game_logic;
mod engine;

use engine::engine_helper::EngineHelper;
use engine::modules_manager::ModulesManager;

use glium::DisplayBuild;
use game_logic::logic_handler::LogicHandler;



fn main() {

    //-----------Faire un handler pour les controls
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();

    let mut logic_manager = LogicHandler::new();
    let mut modules_manager = ModulesManager::new(&display);
    let frame_texture = glium::texture::Texture2d::empty_with_format(&display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap, 800,600).unwrap();
    let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::new(&display, &frame_texture)
        .unwrap();


    let mut engine_helper = EngineHelper::new();

    // let mut key_buf = RefCell::new(vec![]);
    let mut key_buf = "".to_string();
    loop {
        let fps_timer = engine_helper.get_fps();
        // let res =
        //     modules_manager.draw(fps_timer.1, &(&logic_manager).get_buffer(fps_timer), vec![]);
        let local = key_buf.clone();
        let res = modules_manager.draw(fps_timer.1,
                                       &logic_manager.update(fps_timer, &local),
                                       vec![],
                                       &frame_texture,
                                       &mut frame_buffer);

        if res.1.len() > 0 {
            key_buf = res.1[0].to_string().clone();
        }

        if key_buf == "escape_press".to_string() {
            return;
        }

        if logic_manager.get_debug() {
            return;
        }
    }

}
