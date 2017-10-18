extern crate glium;

use engine::sprite::Sprite;
use engine::shader_manager::Shaders;
use engine::sprite_manager::SpriteManager;
use engine::graphics_handler::GraphicsHandler;
use engine::generic_object::GenericObject;
use engine::generic_control::GenericControl;
use engine::text_writer::TextWriter;
use engine::generic_object_type::GenericSpriteType;
use engine::input_manager::InputManager;
use engine::model::Model;
use engine::object_manager::ObjectManager;
use engine::vertex;

pub struct ModulesManager<'a> {
    display: &'a glium::Display,
    program: Vec<glium::program::Program>,
    textures: glium::texture::Texture2dArray,
    frame_texture: glium::texture::Texture2d,
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


        shaders.compile_shaders(&display);

        let frame_texture = glium::texture::Texture2d::empty_with_format(display,
                                                                         glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                         glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();
        let textures = shaders.get_texture_array(&display);
        ModulesManager {
            display: display,
            program: vec![shaders.get_compiled_shader("screen_shader"),
                          shaders.get_compiled_shader("sprite_shader"),
                          shaders.get_compiled_shader("object_shader")],
            textures: textures,
            frame_texture: frame_texture,
        }
    }

    pub fn draw(&mut self,
                delta_time: f64,
                generics_objects: &Vec<Box<GenericObject>>,
                generics_controls: Vec<Box<GenericControl>>,
                ui_texture: &glium::texture::Texture2d,
                frame_buffer: &mut glium::framebuffer::SimpleFrameBuffer,
                thirdd_objects: Vec<(f32, f32, f32)>, time:f64)
                    -> (&ModulesManager, Vec<&str>) {

        let bunch_of_generic_sprite_objects =
        self.generic_sprite_object_interpretor(generics_objects).get_buffers( self.display);

        let bunch_of_thirdd_objects = self.thirdd_object_interpretor(thirdd_objects);
        GraphicsHandler::draw( &self.display,
                                bunch_of_generic_sprite_objects,
                                & self.textures,
                                ui_texture,
                                &self.program,
                                frame_buffer,
                                bunch_of_thirdd_objects, time);
        ( self, vec![])//InputManager::get_input( self.display))
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
                GenericSpriteType::SPRITE => {
                    result_vec.push(Sprite::new(name,
                                                position.0,
                                                position.1,
                                                [1.0, 0.0, 0.0, 1.0],
                                                i.get_texture_id() as u32,
                                                (i.get_size().0, i.get_size().1),
                                                texture_coordinates,
                                                order));
                }

                GenericSpriteType::TEXT => {
                    let text_writer = TextWriter::new(0,
                                                      (256, 256),
                                                      (16, 16),
                                                      0.05,
                                                      (position.0, position.1),
                                                      &name,
                                                      true, order);
                    result_vec.extend_from_slice(&text_writer.get_string(description.as_str()));
                }
            }
        }
        result_vec.sort_by(|a, b| a.order.cmp(&b.order));
        SpriteManager::new(result_vec)
    }

    pub fn thirdd_object_interpretor(&self, thirdd_objects: Vec<(f32, f32, f32)>) -> (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {
        let mut result_vec = Vec::new();

        for e in thirdd_objects {
            result_vec.push(Model::new("cube".to_string(),e.0, e.1, e.2,[1.0, 0.0, 0.0, 0.0],(1.0, 1.0, 1.0)));
        }
        ObjectManager::get_buffers(&self.display, result_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::generic_object::GenericObject;
    use engine::generic_object_type::GenericSpriteType;

    #[derive(Debug)]
    struct ObjTest {
        size: i32,
    }

    impl GenericObject for ObjTest {
        fn get_type(&self) -> GenericSpriteType {
            GenericSpriteType::SPRITE
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
