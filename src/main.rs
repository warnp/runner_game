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
use std::cell::RefCell;
use std::rc::Rc;

use engine::generic_camera::GenericCamera;
use self::cgmath::{Matrix4, Vector3, Rad};
use self::cgmath::prelude::*;

#[derive(Clone)]
struct CubeObj {
    pub name: String,
    pub texture: i32,
    pub size: (f32, f32, f32),
    pub texture_coordinate: ((f32, f32), (f32, f32), (f32, f32), (f32, f32)),
    pub mesh: String,
    pub matrix: Matrix4<f32>,
}


impl GenericObject for CubeObj {
    fn get_type(&self) -> GenericObjectType {
        GenericObjectType::STATIC_MESH
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
    fn get_mesh(&self) -> String {
        self.mesh.clone()
    }
    fn get_matrix(&self) -> Matrix4<f32> {
        self.matrix
    }
}

#[derive(Clone)]
struct CameraExtern {
    position: Matrix4<f32>,
}

impl GenericCamera for CameraExtern {
    fn get_name(&self) -> String {
        "first_cam".to_string()
    }

    fn get_position(&self) -> Matrix4<f32> {
        self.position
    }

    fn set_position(&mut self, position: Matrix4<f32>) {
        self.position = position;
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
    fn box_clone(&self) -> Box<GenericCamera> {
        Box::new((*self).clone())
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

    let cam = CameraExtern { position: Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: 250.0 }) };
    let mut cameras = vec![Box::new(cam) as Box<GenericCamera>];

    let mut pos = 250.0;

    let mut titi_pos = (0.0, 0.0, 0.0);

    let mut toto = CubeObj { matrix: Matrix4::identity(), mesh: "test".to_string(), name: "toto".to_string(), texture: 0, texture_coordinate: ((0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)), size: (1.0, 1.0, 1.0) };
    let mut titi = CubeObj { matrix: Matrix4::identity(), mesh: "wheel".to_string(), name: "titi".to_string(), texture: 0, texture_coordinate: ((0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)), size: (1.0, 1.0, 1.0) };
let mut pouet = 10.0;
    while !close {
        let key_press = engine::controls::key_action::KeyAction {
            key: "A".to_string(),
            action: move |x| {
                let mut plop = vec![];
                for cam in &x {
                    let mut camera = cam.clone();
                    if camera.get_name() == "first_cam" {
                        let pos = camera.get_position() * Matrix4::from_translation(Vector3 { x: 0.0, y: 0.0, z: -0.5 });
                        camera.set_position(pos);
                        plop.push(camera);
                        continue;
                    }
                }

                plop
            },
        };

        let fps_timer = engine_helper.get_fps();


        let local_keys = key_buf.clone();

        let toto_matrix = Matrix4::from_translation(Vector3 { x: -100.0, y: -20.0, z: 0.0 });

        let titi_matrix = Matrix4::from_translation(Vector3 { x: -40.0, y: 0.0, z: 25.0 });

        let f = frames;

        let plop = Matrix4::from_angle_y(Rad(pouet * 0.001));
        pouet = pouet + 1.0;
        let toto_matrix = toto_matrix * plop;

        let titi_matrix = toto_matrix * titi_matrix;


        let mut toto = toto.clone();
        let mut titi = titi.clone();

        toto.matrix = toto_matrix;
        titi.matrix = titi_matrix;

        let toto = Box::new(toto);
        let titi = Box::new(titi);
        let res = modules_manager.draw(fps_timer.1,
                                       &vec![Box::new(Text::new("fps".to_string(), [0.0, 0.0], 255, "Salut".to_string())),
//                                             Box::new(CubeObj { position: (0.0, 0.0, 0.0), name: "test".to_string(), texture: 0, texture_coordinate: ((0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)), size: (1.0, 1.0, 1.0) }),
                                             titi, toto],
                                       vec![Box::new(key_press)],
                                       RefCell::new(cameras),
                                       vec![(0.0, 0.0, 1.0)], frames, &mut events_loop);
        cameras = res.2;
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
//        events_loop.poll_events(|ev| {
//            match ev {
//                glium::glutin::Event::WindowEvent {
//                    event, ..
//                } => match event {
//                    glium::glutin::WindowEvent::Closed => close = true,
//                    glium::glutin::WindowEvent::KeyboardInput {
//                        input, ..
//                    } => {
//                        println!("{:?}", input);
//
//                        if input.scancode == 1 {
//                            close = true;
//                        }
//                    }
//                    _ => (),
//                },
//                _ => (),
//            }
//        });
    }
}
