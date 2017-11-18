#[macro_use]
extern crate glium;
extern crate time;
extern crate rand;
//extern crate rodio;


mod game_logic;
mod engine;

use engine::engine_helper::EngineHelper;
use engine::modules_manager::ModulesManager;
use game_logic::text::Text;
//use engine::input_manager::InputManager;


//use glium::DisplayBuild;
use game_logic::logic_handler::LogicHandler;

//use std::sync::atomic::Ordering;
//use std::sync::Arc;
//use std::thread;

//use std::fs::File;
//use std::io::BufReader;
//use rodio::Source;
//use std::time::Duration;

fn main() {
    let screen_size = (800, 600);

    //    let endpoint = rodio::get_default_endpoint().unwrap();

    //    let file = File::open("./content/5032.wav").unwrap();
    //    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    //    let source = source.repeat_infinite();

    //    rodio::play_raw(&endpoint, source.convert_samples());


    //-----------Faire un handler pour les controls
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_dimensions(screen_size.0,screen_size.1);
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop)
        .unwrap();

    let mut modules_manager = ModulesManager::new(&display);


    let mut engine_helper = EngineHelper::new();

    // Use a key buffer to be able to make some replays (:
    let mut key_buf: Vec<String> = vec!["".to_string()];

    let mut pause = false;
    let mut close = false;
    let mut frames = 0.0;
    while !close {

        let fps_timer = engine_helper.get_fps();


        let local_keys = key_buf.clone();
        let res = modules_manager.draw(fps_timer.1,
                                       &vec![Box::new(Text::new("fps".to_string(),[-0.5, 0.8], 255, "Salut".to_string()))],
                                       vec![],
                                       vec![(0.0, 0.0, 1.0)], frames);

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

        frames = frames + 1.0;
        events_loop.poll_events(|ev| {
            match ev {
                glium::glutin::Event::WindowEvent {
                    event, ..
                } => match event {
                    glium::glutin::WindowEvent::Closed => close = true,
                    glium::glutin::WindowEvent::KeyboardInput {
                        input, ..
                    } => {
                        println!("{:?}", input);

                        if input.scancode == 1 {
                            close = true;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
