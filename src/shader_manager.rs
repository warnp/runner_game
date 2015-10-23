use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct shader_couple {
    pub vertex_shader: &str,
    pub pixel_shader: &str,
}

struct shaders {
    shaders_list: HashMap<String, Box<shader_couple>>,

}

impl shaders {
    pub fn compile_shaders(&self) -> HashMap<String, //shader compile -> program
}
