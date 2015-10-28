use std::collections::HashMap;

extern crate glium;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ShaderCouple<'a> {
    pub vertex_shader: &'a str,
    pub pixel_shader: &'a str,
}

#[derive(Debug)]
pub struct Shaders<'a> {
    pub shaders_list: HashMap<&'a str, ShaderCouple<'a>>,
    compiled_shaders: HashMap<&'a str, Box<glium::program::Program>>,

}

impl <'a>Shaders<'a> {

    pub fn new() -> Shaders<'a> {

        let mut hash = HashMap::new();

        hash.insert("simple_shader", ShaderCouple {
            vertex_shader: r#"
            #version 140

            in vec2 position;
            in vec3 normal;
            in vec4 color;
            in vec2 tex_coords;

            out vec4 colorV;
            out vec3 v_normal;
            out vec2 v_tex_coords;

            uniform mat4 matrix;

            void main(){
                // colorV = color;
                v_tex_coords = tex_coords;
                gl_Position = matrix * vec4(position, 0.0,1.0);
            }
            "#,

            pixel_shader: r#"
            #version 140

            in vec4 colorV;
            in vec2 v_tex_coords;

            out vec4 color;

            uniform sampler2D tex;

            void main(){
                color = texture(tex, v_tex_coords);
            }
            "#
        });

        Shaders
            {
                shaders_list: hash,
                compiled_shaders: HashMap::new(),
            }

    }

    pub fn compile_shaders(&mut self, display: &glium::backend::glutin_backend::GlutinFacade) {
        for (name, s) in self.shaders_list.iter() {
            self.compiled_shaders.insert(name, Box::new(glium::Program::from_source(display, s.vertex_shader, s.pixel_shader, None).unwrap()));
        }

    }

    pub fn get_compiled_shader(&mut self, shader_name: &'a str) -> glium::program::Program {
            *self.compiled_shaders.remove(&shader_name).unwrap()
    }
}


#[cfg(test)]
mod shader_manager_tests {
    use super::*;

    use std::collections::HashMap;
    use glium::backend::Facade;
    use glium::{DisplayBuild, Surface};

    extern crate glium;

    #[test]
    fn should_return_a_hashmap(){
        let shader = Shaders::new();
        let display = glium::glutin::WindowBuilder::new()
                                    .build_glium()
                                    .unwrap();
        let lst_shaders = shader.compile_shaders(&display);
        assert!(lst_shaders.len() == 1);
    }
}
