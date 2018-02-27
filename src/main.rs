#[macro_use]
extern crate glium;
extern crate time;
extern crate rand;
extern crate cgmath;

//extern crate rodio;


mod game_logic;
mod engine;

use engine::engine_helper::EngineHelper;
use engine::modules_manager::ModulesManager;
use game_logic::text::Text;
use engine::generic_object::GenericObject;
use engine::generic_object_type::GenericObjectType;
use game_logic::logic_handler::LogicHandler;

use engine::generic_camera::GenericCamera;
use self::cgmath::{Matrix4, Vector3};

struct CubeObj {
    pub position: (f32, f32, f32),
    pub name: String,
    pub texture: i32,
    pub size: (f32, f32, f32),
    pub texture_coordinate: ((f32, f32), (f32, f32), (f32, f32), (f32, f32)),
}


impl GenericObject for CubeObj {
    fn get_type(&self) -> GenericObjectType {
        GenericObjectType::STATIC_MESH
    }

    fn get_position(&self) -> (f32, f32, f32) {
        self.position
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        "a description!".to_string()
    }

    fn get_texture_id(&self) -> i32 {
        self.texture
    }

    fn get_size(&self) -> (f32, f32, f32) {
        self.size
    }

    fn get_texture_coordinates(&self) -> ((f32, f32), (f32, f32), (f32, f32), (f32, f32)) {
        self.texture_coordinate
    }

    fn get_order(&self) -> u8 {
        0u8
    }
}

struct CameraExtern {}

impl GenericCamera for CameraExtern {
    fn get_name(&self) -> String {
        "first_cam".to_string()
    }

    fn get_position(&self) -> Matrix4<f32> {
        Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: 200.0 })
    }

    fn get_active(&self) -> bool {
        true
    }

    fn get_aspect(&self) -> f32 {
        800.0 / 600.0
    }

    fn get_view_angle(&self) -> f32 {
        65.0
    }
}

fn main() {
    let screen_size = (800, 600);

    //    let endpoint = rodio::get_default_endpoint().unwrap();

    //    let file = File::open("./content/5032.wav").unwrap();
    //    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    //    let source = source.repeat_infinite();

    //    rodio::play_raw(&endpoint, source.convert_samples());


    //-----------Faire un handler pour les controls
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new().with_dimensions(screen_size.0, screen_size.1);
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
        let cam = CameraExtern{};
        let res = modules_manager.draw(fps_timer.1,
                                       &vec![Box::new(Text::new("fps".to_string(), [-0.5, 0.8], 255, "Salut".to_string())),
                                             Box::new(CubeObj { position: (0.0, 0.0, 0.0), name: "test".to_string(), texture: 0, texture_coordinate: ((0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)), size: (1.0, 1.0, 1.0) })],
                                       vec![],
                                       vec![Box::new(cam)],
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
