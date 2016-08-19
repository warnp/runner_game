extern crate glium;
extern crate time;
use engine::vertex::Vertex;
use glium::{DisplayBuild, Surface};
use std::time::Instant;



pub struct GraphicsHandler;

impl GraphicsHandler {

    pub fn draw(display: &glium::backend::glutin_backend::GlutinFacade,
                buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>),
                textures: &glium::texture::Texture2dArray,
                program: &glium::Program) {


        //--------------------------UI-DRAW-START---------------------------//
        // TRANSFORM TO HAVE NICE SPRITE SIZE
        let uniforms = uniform! {
                matrix: [
                    [600.0/800.0, 0.0 , 0.0 , 0.0],
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

        let errors = target.finish().unwrap();

        //--------------------------UI-DRAW-END-----------------------------//
        //-------------------DEFFERED-RENDERING-DRAW-START------------------//
        //-------------------DEFFERED-RENDERING-DRAW-END--------------------//

    }
}
