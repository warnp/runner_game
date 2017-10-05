extern crate glium;
extern crate time;

use engine::vertex::Vertex;
use glium::Surface;
use glium::PolygonMode;
use engine::sprite::Sprite;
use engine::camera::Camera;


pub struct GraphicsHandler;

impl GraphicsHandler {
    pub fn draw(display: &glium::backend::glutin_backend::GlutinFacade,
                ui_buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>),
                objects_textures: &glium::texture::Texture2dArray,
                ui_texture: &glium::texture::Texture2d,
                program: &Vec<glium::program::Program>,
                frame_buffer: &mut glium::framebuffer::SimpleFrameBuffer,
                thirdd_buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>)) {
        //--------------------------3D-DRAW-START---------------------------//
//        let thirdd_vertex_buffer = thirdd_buffers.0;
        let thirdd_index_buffer = thirdd_buffers.1;

        let zfar = 1024.0;
        let znear = 0.1;

        let aspect_ratio = 600.0/800.0;
        let fov: f32 = 3.141592/3.0;
        let f = 1.0 / (fov / 2.0).tan();
        let camera = Camera::new([2.0, -1.0, 1.0], [-2.0, 1.0, 1.0], [0.0, 1.0, 0.0]);

        let thirdd_uniform = uniform!(
            u_matrix: [[1.0,0.0      ,0.0      ,0.0],
                       [0.0      ,1.0,0.0      ,0.0],
                       [0.0      ,0.0      ,1.0,0.0],
                       [0.0     ,0.0      ,0.0      ,1.0f32]]

        );

        let thirdd_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
//            draw_primitives: true,
//            line_width: Some(0.5),
//            polygon_mode: PolygonMode::Line,

            ..Default::default()
        };

        let test = Sprite::new("toto".to_string(),
                                 0.0, 0.0,
                                 [1.0, 0.0, 0.0, 1.0],
                                 1u32,
                                 (1.0, 1.0),
                                 ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)),
                                 0);

        let thirdd_vertex_buffer = glium::VertexBuffer::dynamic(display,&test.vertices).unwrap();


        frame_buffer.clear_color(1.0f32, 1.0f32, 0.0f32, 1.0f32);
        frame_buffer.draw(&thirdd_vertex_buffer, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &program[2], &thirdd_uniform, &thirdd_params ).unwrap();
//        frame_buffer.draw(&thirdd_vertex_buffer, &thirdd_index_buffer, &program[1], &thirdd_uniform, &thirdd_params ).unwrap();

        //---------------------------3D-DRAW-END----------------------------//

        //--------------------------UI-DRAW-START---------------------------//
//        let ui_vertex_buffer = ui_buffers.0;
//        let ui_index_buffer = ui_buffers.1;
//        let ui_uniform = uniform! {
//                matrix: [
//                    [600.0/800.0, 0.0 , 0.0 , 0.0],
//                    [0.0                       , 1.0 , 0.0 , 0.0],
//                    [0.0                       , 0.0 , 1.0 , 0.0],
//                    [0.0                       , 0.0 , 0.0 , 1.0f32],
//                ],
//                tex: objects_textures,
//            };
//
//
//        let ui_params = glium::DrawParameters {
//            blend: glium::Blend::alpha_blending(),
//
//            ..Default::default()
//        };
//        frame_buffer.clear_color(1.0f32, 1.0f32, 1.0f32, 1.0f32);
//        frame_buffer.draw(&ui_vertex_buffer, &ui_index_buffer, &program[1], &ui_uniform, &ui_params).unwrap();

        //--------------------------UI-DRAW-END-----------------------------//

        //-------------------DEFFERED-RENDERING-DRAW-START------------------//
        let uniforms = uniform! {
            matrix: [
                        [1.0, 0.0 , 0.0 , 0.0],
                        [0.0, 1.0 , 0.0 , 0.0],
                        [0.0, 0.0 , 1.0 , 0.0],
                        [0.0, 0.0 , 0.0 , 1.0f32],
            ],
             ui_texture: ui_texture,
        };

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        //La taille de l'écran est 2 parce que la taille de l'écran va de -1 à 1 soit 2
        let screen = Sprite::new("screen".to_string(),
                                 0.0, 0.0,
                                 [1.0, 0.0, 0.0, 1.0],
                                 1u32,
                                 (2.0, 2.0),
                                 ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)),
                                 0);

        let vertex_buffer = glium::VertexBuffer::dynamic(display, &screen.vertices).unwrap();
        let index_buffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                                          &screen.indices)
            .unwrap();

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &index_buffer, &program[0], &uniforms, &Default::default())
            .unwrap();

        let errors = target.finish().unwrap();

        //-------------------DEFFERED-RENDERING-DRAW-END--------------------//
    }
}
