use sprite::Sprite;
use shader_manager::{Shaders, ShaderCouple};
use sprite_manager::SpriteManager;
use graphics_handler::GraphicsHandler;

pub struct ModulesManager;

impl ModulesManager {
    pub fn start(sprite_list: Vec<Sprite>, shaders: Shaders) {
        let display = GraphicsHandler::init(768, 1024);
        let program = GraphicsHandler::compile_shaders(&display, vec![], "simple_shader");

        loop {
            GraphicsHandler::draw(&display);
        }
    }
}
