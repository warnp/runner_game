#[macro_use]
extern crate glium;
extern crate time;
extern crate rand;
//extern crate rodio;


mod game_logic;
mod engine;

use engine::engine_helper::EngineHelper;
use engine::modules_manager::ModulesManager;
//use engine::input_manager::InputManager;


use glium::DisplayBuild;
use game_logic::logic_handler::LogicHandler;

//use std::sync::atomic::Ordering;
//use std::sync::Arc;
//use std::thread;

//use std::fs::File;
//use std::io::BufReader;
//use rodio::Source;
//use std::time::Duration;

fn main() {
//    let endpoint = rodio::get_default_endpoint().unwrap();

//    let file = File::open("./content/5032.wav").unwrap();
//    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
//    let source = source.repeat_infinite();

//    rodio::play_raw(&endpoint, source.convert_samples());


    //-----------Faire un handler pour les controls
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();

    let mut modules_manager = ModulesManager::new(&display);
    let frame_texture = glium::texture::Texture2d::empty_with_format(&display,
                                                                     glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                     glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();
    let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::new(&display, &frame_texture)
        .unwrap();


    let mut engine_helper = EngineHelper::new();

    // Use a key buffer to be able to make some replays (:
    let mut key_buf: Vec<String> = vec!["".to_string()];

    let mut pause = false;

    loop {
        let fps_timer = engine_helper.get_fps();


        let local_keys = key_buf.clone();
        let res = modules_manager.draw(fps_timer.1,
                                       &vec![],
                                       vec![],
                                       &frame_texture,
                                       &mut frame_buffer,
                                        vec![(0.0,0.0,1.0)]);

        if res.1.len() > 0 {
            key_buf.push(res.1[0].to_string().clone());
        };

        if key_buf.last().unwrap() == "a_press" {
            key_buf = vec!["".to_string()];
            pause = !pause;
        }

        if key_buf.contains(&("escape_press".to_string())) {
            return;
        };

    }
}
