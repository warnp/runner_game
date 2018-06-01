extern crate glium;
extern crate cgmath;

use engine::graphic::sprite::Sprite;
use engine::graphic::shader_manager::Shaders;
use engine::graphic::sprite_manager::SpriteManager;
use engine::graphic::graphics_handler::GraphicsHandler;
use engine::graphic::generic_object::GenericObject;
use engine::graphic::text_writer::TextWriter;
use engine::graphic::generic_object_type::GenericObjectType;
use engine::graphic::model::{ Model, Light};
use engine::graphic::camera::Camera;
use engine::graphic::object_manager::ObjectManager;
use std::cell::RefCell;
use self::cgmath::{Matrix, Matrix4};
use engine::graphic::generic_camera::GenericCamera;
use engine::controls::key_action::AnyKeyAction;

pub struct ModulesManager<'a> {
    display: &'a glium::Display,
    textures: glium::texture::Texture2dArray,
    shader_manager: Shaders<'a>,
    object_manager: ObjectManager,
    event_loop: glium::glutin::EventsLoop,

}

impl<'a> ModulesManager<'a> {
    pub fn new(display: &glium::Display) -> ModulesManager {
        let mut shaders = Shaders::new(vec![&include_bytes!("../../content/VFKM2.png")[..],
                                            &include_bytes!("../../content/11532.png")[..],
                                            &include_bytes!("../../content/NatureForests.png")[..],
                                            &include_bytes!("../../content/hero.png")[..],
                                            &include_bytes!("../../content/background.png")[..],
                                            &include_bytes!("../../content/game_over.png")[..]],
                                       &display);


        let textures = shaders.get_texture_array(&display);
        let mut objects = ObjectManager::new();
        //Ca charge tout les objets disponibles, pas glop
//        objects.preload_object_list();

        ModulesManager {
            display: display,
            textures: textures,
            shader_manager: shaders,
            object_manager: objects,
            event_loop: glium::glutin::EventsLoop::new(),

        }
    }

    pub fn draw(&mut self,
                delta_time: f64,
                generics_objects: &Vec<Box<GenericObject>>,
                generics_controls: Vec<Box<AnyKeyAction>>,
                generics_cameras: RefCell<Vec<Box<GenericCamera>>>,
                thirdd_objects: Vec<(f32, f32, f32)>, time: f64, event_loop: &mut glium::glutin::EventsLoop)
                -> (&ModulesManager, Vec<&str>, Vec<Box<GenericCamera>>) {
        let t1 = generics_cameras.borrow();
        if t1.is_empty() {
            panic!("No camera found!")
        }

        //For debug!
        self.shader_manager.update_program_list();
        self.object_manager.update_realtime_objects_list();

//        let available_objects = self.object_manager.get_objects_availables();
        //Handle controls
        let mut cams = generics_cameras.borrow().to_vec();
        for c in generics_controls {
            event_loop.poll_events(|ev| {
                match ev {
                    glium::glutin::Event::WindowEvent {
                        event, ..
                    } => match event {
                        glium::glutin::WindowEvent::KeyboardInput {
                            input, ..
                        } => {
                            println!("{:?}", input);

                            if input.scancode == 72 {
                                cams = c.execute_action(generics_cameras.borrow().to_vec());
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            });
        }


        let bunch_of_generic_sprite_objects =
            self.generic_sprite_object_interpretor(generics_objects).get_buffers(self.display);

        //Retourne le nom, la position et la texture de l'objet
        let bunch_of_generic_mesh = generics_objects.iter()
            .filter(|x| x.get_type() == GenericObjectType::STATIC_MESH)
            .map(|x| (x.get_name(), x.get_matrix(), x.get_texture_id(), x.get_mesh()))
            .collect::<Vec<(String, Matrix4<f32>, i32, String)>>();


        self.object_manager.update_object_list(bunch_of_generic_mesh);
        //Camera handler
        let cams_clone = cams.clone();
        let camera_conf = cams_clone.get(0).unwrap();
        let camera = Camera {
            name: camera_conf.get_name(),
            position: camera_conf.get_position(),
            active: camera_conf.get_active(),
            aspect: camera_conf.get_aspect(),
            view_angle: camera_conf.get_view_angle(),
            rotation: Camera::generate_rotation(0.0, 0.0, 0.0),
        };

        self.object_manager.update_loaded_model_list(camera.position.row(2), generics_objects.iter()
            .map(|element| element.get_name()).collect::<Vec<String>>());
        self.object_manager.load_models_into_buffer();
        let model_to_load = self.object_manager.available_models
            .iter()
            .map(|x| RefCell::new(Box::new(x.clone())) as RefCell<Box<Model>>)
            .collect::<Vec<RefCell<Box<Model>>>>();


        GraphicsHandler::draw(&self.display,
                              bunch_of_generic_sprite_objects,
                              &self.textures,
                              &self.shader_manager.compiled_programms,
                              model_to_load,
//                              bunch_of_thirdd_objects,
                              vec![Light { name: "lumiere".to_string(), intensity: 128, position: (50.0, 10.0, 0.0, 0.0), attenuation: (1.0, 0.00124, 0.00001), color: (1.0, 1.0, 1.0), radius: 200.0 },
//                                       Light{name: "lumiere".to_string(), intensity:128, position:(-50.0,10.0,0.0,0.0),attenuation:(1.0,0.00124,0.00001), color:(1.0,1.0,1.0),radius:200.0}
                              ],
                              camera,
                              time);
        (self, vec![], cams)//InputManager::get_input( self.display))
    }

    pub fn generic_sprite_object_interpretor(&self,
                                             generic_object: &Vec<Box<GenericObject>>)
                                             -> SpriteManager {
        let mut result_vec = Vec::new();
//        let mut name: String;
//        let mut position: (f32, f32, f32);
//        let mut description: String;
//        let mut texture_coordinates: ((f32, f32), (f32, f32), (f32, f32), (f32, f32));
//        let mut order: u8;
        //TODO Ajouter un ordonnanceur de sprites
        for i in generic_object {
            let name = i.get_name();

            let pos_vec1 = i.get_matrix().row(0);
            let pos_vec2 = i.get_matrix().row(1);

            let position = (pos_vec1.w, pos_vec2.w, 0.0);
            let description = i.get_description();
            let texture_coordinates = i.get_texture_coordinates();
            let order = i.get_order();
            match i.get_type() {
                GenericObjectType::SPRITE => {
                    result_vec.push(Sprite::new(name,
                                                position.0,
                                                position.1,
                                                [1.0, 0.0, 0.0, 1.0],
                                                i.get_texture_id() as u32,
                                                (i.get_size().0, i.get_size().1),
                                                texture_coordinates,
                                                order));
                }

                GenericObjectType::TEXT => {
                    let text_writer = TextWriter::new(0,
                                                      (256, 256),
                                                      (16, 16),
                                                      0.1,
                                                      (position.0, position.1),
                                                      &name,
                                                      true, order);
                    result_vec.extend_from_slice(&text_writer.get_string(description.as_str()));
                }
                GenericObjectType::STATIC_MESH => {}
            }
        }
        result_vec.sort_by(|a, b| a.order.cmp(&b.order));
        SpriteManager::new(result_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::generic_object::GenericObject;
    use engine::generic_object_type::GenericObjectType;

    #[derive(Debug)]
    struct ObjTest {
        size: i32,
    }

    impl GenericObject for ObjTest {
        fn get_type(&self) -> GenericObjectType {
            GenericObjectType::SPRITE
        }

        fn get_position(&self) -> (f32, f32, f32) {
            (0.0, 0.0, 0.0)
        }
        fn get_name(&self) -> String {
            "Test".to_string()
        }
        fn get_description(&self) -> String {
            "This is a test description".to_string()
        }
        fn get_texture_id(&self) -> i32 {
            0
        }
        fn get_size(&self) -> (f32, f32, f32) {
            (0.0, 0.0, 0.0)
        }
        fn get_texture_coordinates(&self) -> ((f32, f32), (f32, f32)) {
            ((0.0, 0.0), (0.0, 0.0))
        }
    }
}
