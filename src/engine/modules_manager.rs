extern crate glium;
extern crate cgmath;

use engine::sprite::Sprite;
use engine::shader_manager::Shaders;
use engine::sprite_manager::SpriteManager;
use engine::graphics_handler::GraphicsHandler;
use engine::generic_object::GenericObject;
use engine::generic_control::GenericControl;
use engine::text_writer::TextWriter;
use engine::generic_object_type::GenericObjectType;
use engine::input_manager::InputManager;
use engine::model::{Cube, Model, Light, Lod};
use engine::camera::Camera;
use engine::object_manager::ObjectManager;
use engine::vertex;
use std::sync::mpsc::Receiver;
use self::cgmath::{Matrix, Matrix4, Vector3};
use engine::generic_camera::GenericCamera;


pub struct ModulesManager<'a> {
    display: &'a glium::Display,
    textures: glium::texture::Texture2dArray,
    shader_manager: Shaders<'a>,
    object_manager: ObjectManager,
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
        objects.preload_object_list();

        println!("Models {:#?}", objects.available_models);


        ModulesManager {
            display: display,
            textures: textures,
            shader_manager: shaders,
            object_manager: objects,
        }
    }

    pub fn draw(&mut self,
                delta_time: f64,
                generics_objects: &Vec<Box<GenericObject>>,
                generics_controls: Vec<Box<GenericControl>>,
                generics_cameras: Vec<Box<GenericCamera>>,
                thirdd_objects: Vec<(f32, f32, f32)>, time: f64)
                -> (&ModulesManager, Vec<&str>) {

        if generics_cameras.is_empty() {
            panic!("No camera found!")
        }

        //For debug!
        self.shader_manager.update_program_list();
        self.object_manager.update_realtime_objects_list();

//        let available_objects = self.object_manager.get_objects_availables();



        let bunch_of_generic_sprite_objects =
            self.generic_sprite_object_interpretor(generics_objects).get_buffers(self.display);

        //        let bunch_of_thirdd_objects = self.thirdd_object_interpretor(thirdd_objects);
//        let bunch_of_thirdd_objects = (glium::VertexBuffer::new(self.display, &teapot::VERTICES).unwrap(), glium::VertexBuffer::new(self.display, &teapot::NORMALS).unwrap(), glium::IndexBuffer::new(self.display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap());
        let camera_conf = generics_cameras.get(0).unwrap();
        let camera = Camera {
            name: camera_conf.get_name(),
            position: camera_conf.get_position(),
            active: camera_conf.get_active(),
            aspect: camera_conf.get_aspect(),
            view_angle: camera_conf.get_view_angle(),
            rotation: Camera::generate_rotation(0.0,0.0,0.0)
        };
//        let dist = camera_position.distance(model_position);
        self.object_manager.update_loaded_model_list(camera.position.row(2),generics_objects.iter()
            .map(|element| element.get_name()).collect::<Vec<String>>());
        self.object_manager.load_models_into_buffer();
        let model_to_load = self.object_manager.available_models.iter().map(|x| Box::new(x.clone()) as Box<Model>).collect::<Vec<Box<Model>>>();


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
        (self, vec![])//InputManager::get_input( self.display))
    }

    pub fn generic_sprite_object_interpretor(&self,
                                             generic_object: &Vec<Box<GenericObject>>)
                                             -> SpriteManager {
        let mut result_vec = Vec::new();
        let mut name: String;
        let mut position: (f32, f32, f32);
        let mut description: String;
        let mut texture_coordinates: ((f32, f32), (f32, f32), (f32, f32), (f32, f32));
        let mut order: u8;
        //TODO Ajouter un ordonnanceur de sprites
        for i in generic_object {
            name = i.get_name();
            position = i.get_position();
            description = i.get_description();
            texture_coordinates = i.get_texture_coordinates();
            order = i.get_order();

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

    pub fn thirdd_object_interpretor(&self, thirdd_objects: Vec<(f32, f32, f32)>) -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {
        let mut result_vec = Vec::new();

        for e in thirdd_objects {
            result_vec.push(Cube::new("cube".to_string(), e.0, e.1, e.2, [1.0, 0.0, 0.0, 0.0], (1.0, 1.0, 1.0)));
        }
        ObjectManager::get_buffers(&self.display, result_vec)
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
