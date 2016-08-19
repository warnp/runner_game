extern crate glium;
use engine::sprite::Sprite;
use engine::shader_manager::Shaders;
use engine::sprite_manager::SpriteManager;
use engine::graphics_handler::GraphicsHandler;
use glium::DisplayBuild;
use engine::generic_object::GenericObject;
use engine::generic_control::GenericControl;
use engine::text_writer::TextWriter;
use engine::generic_object_type::GenericObjectType;
use engine::input_manager::InputManager;
use engine::frame_buffer_manager::FrameBufferManager;
use glium::Surface;
use glium::backend::Facade;
// use std::boxed::Box;

pub struct ModulesManager<'a>{
    display: &'a glium::backend::glutin_backend::GlutinFacade,
    program: glium::program::Program,
    textures: glium::texture::Texture2dArray,
    // frame_buffer: glium::framebuffer::SimpleFrameBuffer<'a>,
    frame_texture: glium::texture::Texture2d,
    // sprite_manager: SpriteManager<'a>,

}

impl<'a> ModulesManager<'a> {

    pub fn new(display: &glium::backend::glutin_backend::GlutinFacade) -> ModulesManager{

          let mut shaders = Shaders::new(vec![&include_bytes!("../../content/VFKM2.png")[..],
                                                              &include_bytes!("../../content/11532.png")[..],
                                                              &include_bytes!("../../content/NatureForests.png")[..],
                                                              &include_bytes!("../../content/hero.png")[..]], &display);
          shaders.compile_shaders(&display);

          let frame_texture = glium::texture::Texture2d::empty_with_format(display, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, 800,600).unwrap();
          let textures = shaders.get_texture_array(&display);
          ModulesManager{
                display: display,
                program: shaders.get_compiled_shader("simple_shader"),
                textures: textures,
                frame_texture: frame_texture,
                // frame_buffer: frame_buffer,
                // sprite_manager: buffers,
        }
    }



    pub fn draw(&mut self,
         delta_time: f64,
         generics_objects: &Vec<Box<GenericObject>>,
         generics_controls: Vec<Box<GenericControl>>,
         frame_buffer: &mut glium::framebuffer::SimpleFrameBuffer) -> (&ModulesManager, Vec<&str>) {

        // match self.frame_buffer {
        //     Some(x) => (*x).init_frame_buffer(&self.display),
        //     None => self.frame_buffer =  Some(Box::new(FrameBufferManager::new(&self.display))),
        // }

        let bunch_of_generic_objects = self.generic_object_interpretor(generics_objects).get_buffers(self.display);

        //--------------------TEST--------------------//

        let lighting_program = glium::Program::from_source(self.display,
       // vertex shader
       "
           #version 140

           in vec2 position;

           void main() {
               gl_Position = vec4(position, 0.0, 1.0);
           }
       ",

       // fragment shader
       "
           #version 140

           out vec4 outColor;

           void main(){
               outColor = vec4(1.0,1.0,1.0,1.0);
           }
       ",

       // geometry shader
       None)
       .unwrap();

   let uniforms = uniform! {
           position: [0.0,0.0],
       };

        frame_buffer.clear_color(0.0f32, 0.0f32, 0.0f32, 0.0f32);
        frame_buffer.draw(&bunch_of_generic_objects.0,&bunch_of_generic_objects.1,&lighting_program, &uniforms,  &Default::default()).unwrap();


        //--------------------FIN-TEST----------------//

         GraphicsHandler::draw(&self.display,
              bunch_of_generic_objects,
              &self.textures,
              &self.program);
        (self,InputManager::get_input(self.display))
    }

    pub fn generic_object_interpretor(&self, generic_object:  &Vec<Box<GenericObject>>) -> SpriteManager{
        let mut result_vec = Vec::new();
        let mut name : String;
        let mut position: (f32,f32,f32);
        let mut description : String;
        for i in generic_object {
            name = i.get_name();
            position = i.get_position();
            description = i.get_description();
            match i.get_type() {
                GenericObjectType::Sprite => {
                    result_vec.push(Sprite::new(name, position.0,position.1,[1.0,0.0,0.0,1.0],i.get_texture_id() as u32,(0.1,0.1),0));
                },

                GenericObjectType::Text => {
                        let text_writer = TextWriter::new(0,(256,256),(16,16),0.05,(position.0,position.1),&name, true);
                        result_vec.extend_from_slice(&text_writer.get_string(description.as_str()));
                },

            }

        }
        SpriteManager::new(result_vec)
    }

}

#[cfg(test)]
mod tests{
    use super::*;
    use engine::generic_object::GenericObject;
    use engine::generic_object_type::GenericObjectType;

#[derive(Debug)]
struct ObjTest {
    size: i32,
}
impl GenericObject for ObjTest {
    fn get_type(&self) -> GenericObjectType {
        GenericObjectType::Sprite
    }

    fn get_position(&self) -> (f32,f32,f32){
        (0.0,0.0,0.0)
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
}

    #[test]
    fn should_return_modules_manager(){
        let mut modules_manager = ModulesManager::new();

        let new_mod = modules_manager.draw(5.0, &vec![], vec![]);

//TODO need to find a way to test that
        // match new_mod {
        //     Some(x) => assert!(true),
        //     None => assert!(false)
        // }
        // assert!(&modules_manager == &new_mod);
        assert!(false);
    }

    #[test]
    fn should_interpret_generic_object(){
        let modules_manager = ModulesManager::new();

        let object_list = modules_manager.generic_object_interpretor(&vec![Box::new(ObjTest{size: 1})]);
        assert!(object_list.get_sprite_list().len() == 1);
    }
#[ignore]
    #[test]
    fn should_send_input_messages(){
        let modules_manager= ModulesManager::new();
        // let command_list = modules_manager.get_inputs();
        // assert_eq!(command_list.len(), 0);
    }
}
