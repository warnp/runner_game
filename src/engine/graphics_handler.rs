extern crate glium;
use engine::vertex::Vertex;
use glium::{DisplayBuild, Surface};


pub struct GraphicsHandler;

impl GraphicsHandler {
    // pub fn init(screen_height: f32,
    //             screen_width: f32)
    //             -> &glium::backend::glutin_backend::GlutinFacade {
    //
    //     glium::glutin::WindowBuilder::new()
    //         .with_vsync()
    //         .with_dimensions(screen_width as u32, screen_height as u32)
    //         .build_glium()
    //         .unwrap()
    //
    // }

    // pub fn compile_shaders(display: &glium::backend::glutin_backend::GlutinFacade, textures_list: Vec<&str>, shader_name: &str) -> glium::program::Program {
    // let mut shaders_bytes : Vec<&[u8]>;
    //
    // for texture_name in textures_list {
    // shaders_bytes.push(&include_bytes!(shader_name));
    // }
    //
    // let mut shaders = Shaders::new(shaders_bytes);
    // shaders.compile_shaders(&display);
    //
    // shaders.get_compiled_shader(shader_name);
    // }

    pub fn draw(display: &glium::backend::glutin_backend::GlutinFacade,
                buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>),
                textures: &glium::texture::Texture2dArray,
                program: &glium::Program) {

        // TRANSFORM TO HAVE NICE SPRITE SIZE
        let uniforms = uniform! {
                matrix: [
                    [768.0/1024.0, 0.0 , 0.0 , 0.0],
                    [0.0                       , 1.0 , 0.0 , 0.0],
                    [0.0                       , 0.0 , 1.0 , 0.0],
                    [0.0                       , 0.0 , 0.0 , 1.0f32],
                ],
                tex: textures,
            };

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let vertex_buffer = buffers.0;
        let index_buffer = buffers.1;

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &index_buffer, program, &uniforms, &params)
              .unwrap();

        target.finish().unwrap();


        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Escape)) => {
                    return
                }
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }

    }
}
