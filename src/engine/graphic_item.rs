use engine::layer_type::LayerType;

pub trait GraphicItem {
    fn get_position(&self) -> [f32; 2];
    fn get_layer(&self) -> LayerType;
    // fn get_vertex_shader(&self) -> &str;
    // fn get_fragment_shader(&self) -> &str;

    // fn get_texture(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> Result<glium::texture::texture2d::Texture2d, glium::texture::TextureCreationError>;
}
