extern crate glium;
extern crate time;

extern crate cgmath;

use std::cell::RefCell;
use std::collections::HashMap;
use engine::vertex::{Vertex, Normal, TexCoords};
use glium::Surface;
use glium::PolygonMode;
use engine::sprite::Sprite;
use engine::camera::Camera;
use engine::model::{StaticMesh, Model, Light};
use engine::matrix_helper::MatrixHelper;
use self::cgmath::{Matrix4, Vector3, Point3};
use self::cgmath::prelude::*;
use self::cgmath::conv::*;
use self::cgmath::perspective;
use self::cgmath::PerspectiveFov;
use self::cgmath::{Deg, Rad};
use self::cgmath::MetricSpace;
use std::ops::Mul;

pub struct GraphicsHandler;

impl GraphicsHandler {
    pub fn draw(display: &glium::Display,
                ui_buffers: (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>),
                objects_textures: &glium::texture::Texture2dArray,
                programs: &HashMap<String, Box<glium::Program>>,
                models: Vec<RefCell<Box<Model>>>,
//                instancied_thirdd_buffers: (glium::VertexBuffer<Vertex>, glium::VertexBuffer<Normal>, glium::IndexBuffer<u16>),
                lights: Vec<Light>,
                camera: Camera,
                time: f64) {
        //--------------------------BUFFER-INIT-----------------------------//
        let ui_texture = &glium::texture::Texture2d::empty_with_format(display,
                                                                       glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                       glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();
        let diffuse_texture = &glium::texture::Texture2d::empty_with_format(display,
                                                                            glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                            glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();

        let normal_texture = &glium::texture::Texture2d::empty_with_format(display,
                                                                           glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                           glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();
        let position_texture = &glium::texture::Texture2d::empty_with_format(display,
                                                                             glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                             glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();
        let depth_text = glium::texture::depth_texture2d::DepthTexture2d::empty_with_format(display, glium::texture::DepthFormat::F32, glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();

        let light_texture = &glium::texture::Texture2d::empty_with_format(display,
                                                                          glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                          glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();

        let mut light_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(display, light_texture, &depth_text).unwrap();
        let mut ui_buffer = glium::framebuffer::SimpleFrameBuffer::new(display, ui_texture).unwrap();

        let output = &[("diffuse_output", diffuse_texture), ("normal_output", normal_texture), ("position_output", position_texture)];
        let mut output_buffer = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(display, output.iter().cloned(), &depth_text).unwrap();

//        let thirdd_vertex_buffer = instancied_thirdd_buffers.0;
//        let thirdd_normal_buffer = instancied_thirdd_buffers.1;
//        let thirdd_index_buffer = instancied_thirdd_buffers.2;

        let zfar = 1024.0;
        let znear = 0.1;

        let aspect_ratio = 600.0 / 800.0;
        let pi = 3.141592;
        let fov: f32 = pi / 3.0;
        let f = 1.0 / (fov / 2.0).tan();

        //--------------------------DIFFUSE-START---------------------------//
        let world = Matrix4::identity();


        let proj_view = camera.fps(1.0, 2000.0);
        let camera_position = camera.position.row(2);
        let matrix = proj_view.mul(world);

        let matrix: [[f32; 4]; 4] = array4x4(matrix);
        let world: [[f32; 4]; 4] = array4x4(world);
        let thirdd_uniform = uniform!(
            u_matrix: matrix,
            u_world: world,

        );

        let thirdd_params = glium::DrawParameters {
//                        polygon_mode: PolygonMode::Line,
//            draw_primitives: true,
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };


        //---------------------------DIFFUSE-END---------------------------//

        //---------------------------NORMAL-START---------------------------//
        //---------------------------NORMAL-END---------------------------//

        //---------------------------SPECULAR-START---------------------------//
        //---------------------------SPECULAR-END---------------------------//

        //---------------------------STENCIL-START---------------------------//
        //---------------------------STENCIL-END---------------------------//

        //---------------------------DRAW-GEOMETRY-START----------------------//
        output_buffer.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        match programs.get("object_shader") {
            Some(t) => {
//                output_buffer.draw((&thirdd_vertex_buffer, &thirdd_normal_buffer), &thirdd_index_buffer, &t, &thirdd_uniform, &thirdd_params).unwrap();
            }
            None => ()
        }

        for model in models {

//Rotation
//            let model_matrix = model.borrow().get_matrix().mul(Matrix4::from_angle_y(Rad((time as f32 * 0.001))));
//            let world = Matrix4::from_angle_y(Rad((-time as f32 * 0.001))).invert().unwrap();
            let model_matrix = model.borrow().get_matrix();
            let world = Matrix4::identity();
            let model_matrix = proj_view.mul(model_matrix);
            let model_matrix = model_matrix * Matrix4::from_scale(10.0);
            let world: [[f32; 4]; 4] = array4x4(world);
            let matrix: [[f32; 4]; 4] = array4x4(model_matrix);

            let model_uniform = uniform!(
                u_matrix: matrix,
                u_world: world,
                tex: objects_textures,

            );

            let buff = model.borrow().get_buffer(display);


            match programs.get("object_shader") {
                Some(t) => {
                    output_buffer.draw(&buff.0, &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &t, &model_uniform, &thirdd_params).unwrap();
                }
                None => ()
            }
        }
        //---------------------------DRAW-GEOMETRY-END----------------------//


        //---------------------------INIT-SCREEN-START----------------------//
        let ortho_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];


        //La taille de l'écran est 2 parce que la taille de l'écran va de -1 à 1 soit 2
        let screen = Sprite::new("screen".to_string(),
                                 0.0, 0.0,
                                 [1.0, 0.0, 0.0, 1.0],
                                 1u32,
                                 (2.0, 2.0),
                                 ((0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)),
                                 0);

        let quad_vertex_buffer = glium::VertexBuffer::dynamic(display, &screen.vertices).unwrap();
        let quad_index_buffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                                               &screen.indices)
            .unwrap();
        //---------------------------INIT-SCREEN-END------------------------//
        //---------------------------DRAW-LIGHTS-START----------------------//

        let light_params = glium::DrawParameters {
            //depth_function: glium::DepthFunction::IfLessOrEqual,
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One,
                },
                constant_value: (1.0, 1.0, 1.0, 1.0),
            },
            ..Default::default()
        };

        light_buffer.clear_color(0.0, 0.0, 0.0, 0.0);
        for light in lights {
            let light_uniform = uniform!(
                matrix: ortho_matrix,
                position_texture: position_texture,
                normal_texture: normal_texture,
                light_position: light.position,
                light_attenuation: light.attenuation,
                light_color: light.color,
                light_radius: light.radius,
            );

            match programs.get("light_shader") {
                Some(t) => {
                    light_buffer.draw(&quad_vertex_buffer, &quad_index_buffer, &t, &light_uniform, &light_params).unwrap();
                }
                None => ()
            }
        }
        //---------------------------DRAW-LIGHTS-END----------------------//


        //--------------------------UI-DRAW-START---------------------------//
        let ui_vertex_buffer = ui_buffers.0;
        let ui_index_buffer = ui_buffers.1;
        let ui_uniform = uniform! {
                                matrix: [
                                    [600.0/800.0, 0.0 , 0.0 , 0.0],
                                    [0.0                       , 1.0 , 0.0 , 0.0],
                                    [0.0                       , 0.0 , 1.0 , 0.0],
                                    [0.0                       , 0.0 , 0.0 , 1.0f32],
                                ],
                                tex: objects_textures,
                            };


        let ui_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),

            ..Default::default()
        };
        ui_buffer.clear_color(0.0f32, 0.0f32, 0.0f32, 0.0f32);

        match programs.get("sprite_shader") {
            Some(t) => {
                ui_buffer.draw(&ui_vertex_buffer, &ui_index_buffer, &t, &ui_uniform, &ui_params).unwrap();
            }
            None => ()
        }


        //--------------------------UI-DRAW-END-----------------------------//

        //-------------------DEFFERED-RENDERING-DRAW-START------------------//
        let uniforms = uniform! {
            matrix: ortho_matrix,
             diffuse_texture: diffuse_texture,
             light_texture: light_texture,
             ui_texture: ui_texture,
        };

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 1.0, 1.0);

        match programs.get("screen_shader") {
            Some(program) => {
                target.draw(&quad_vertex_buffer, &quad_index_buffer, program, &uniforms, &Default::default())
                    .unwrap();
            }
            None => ()
        }

        let errors = target.finish().unwrap();

        //-------------------DEFFERED-RENDERING-DRAW-END--------------------//
    }

//    fn get_model_matrix(model: Model)-> Matrix4<f32> {
//        match model.
//    }
}
