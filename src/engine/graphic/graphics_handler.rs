extern crate glium;
extern crate time;

extern crate cgmath;

use std::cell::RefCell;
use std::collections::HashMap;
use engine::graphic::vertex::Vertex;
use glium::Surface;
use engine::graphic::sprite::Sprite;
use engine::graphic::camera::Camera;
use engine::graphic::model::{Model, Light};
use self::cgmath::{Matrix4, Vector3};
use self::cgmath::prelude::*;
use self::cgmath::conv::*;
use std::ops::Mul;

pub struct GraphicsHandler;

impl GraphicsHandler {
    //        match model.
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
        let light_depth_text = glium::texture::depth_texture2d::DepthTexture2d::empty_with_format(display, glium::texture::DepthFormat::F32, glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();
        let depth_text = glium::texture::depth_texture2d::DepthTexture2d::empty_with_format(display, glium::texture::DepthFormat::F32, glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();
        let shadow_depth_text = glium::texture::depth_texture2d::DepthTexture2d::empty_with_format(display, glium::texture::DepthFormat::F32, glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();

        let light_texture = &glium::texture::Texture2d::empty_with_format(display,
                                                                          glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                          glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();

        let shadow_texture = &glium::texture::DepthTexture2d::empty(display, 1024, 1024).unwrap();

        let shadow_render_texture = &glium::texture::Texture2d::empty_with_format(display,
                                                                                  glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                                  glium::texture::MipmapsOption::NoMipmap, 800, 600).unwrap();

        let mut light_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(display, light_texture, &light_depth_text).unwrap();
        let mut ui_buffer = glium::framebuffer::SimpleFrameBuffer::new(display, ui_texture).unwrap();
        let mut shadow_buffer = glium::framebuffer::SimpleFrameBuffer::depth_only(display, shadow_texture).unwrap();
        let mut shadow_render_buffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(display, shadow_render_texture, &shadow_depth_text).unwrap();

        let output = &[("diffuse_output", diffuse_texture), ("normal_output", normal_texture), ("position_output", position_texture)];
        let mut output_buffer = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(display, output.iter().cloned(), &depth_text).unwrap();

        let zfar = 1024.0;
        let znear = 0.1;

        let aspect_ratio = 600.0 / 800.0;
        let pi = 3.141592;
        let fov: f32 = pi / 3.0;
        let f = 1.0 / (fov / 2.0).tan();

        //--------------------------DIFFUSE-START---------------------------//
        let world_matrix = Matrix4::identity();


        let proj_view = camera.fps(1.0, 2000.0);
        let camera_position = camera.position.row(2);
        let world_view_projection_matrix = proj_view.mul(world_matrix);

        let world_array: [[f32; 4]; 4] = array4x4(world_matrix);

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


        for model in &models {
//Rotation
            let model_borrowed = model.borrow();


            let model_matrix = model_borrowed.get_matrix();
            let matrix: [[f32; 4]; 4] = array4x4(model_matrix.invert().unwrap());

            let converted: [f32; 3] = Vector3::new(0.5, 0.7, 1.0).into();

            let model_uniform = uniform!(
                model: matrix,
                u_worldViewProjection: array4x4(world_view_projection_matrix * model_matrix),
                u_world: world_array,
                tex: objects_textures,
                reverse_light_direction: converted,
            );

            let buff = glium::VertexBuffer::new(display, &model_borrowed.get_vertices()).unwrap();

            match programs.get("object_shader") {
                Some(t) => {
                    output_buffer.draw(&buff, &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &t, &model_uniform, &thirdd_params).unwrap();
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

        //La taille de l'écran est 2 parce que la taille de l'écran va de -1 à 1 soit 2
        let debug_screen = Sprite::new("screen".to_string(),
                                       0.0, 0.0,
                                       [1.0, 0.0, 0.0, 1.0],
                                       1u32,
                                       (1.0, 1.0),
                                       ((0.0, 1.0), (1.0, 1.0), (1.0, 0.0), (0.0, 0.0)),
                                       0);

        let debug_quad_vertex_buffer = glium::VertexBuffer::dynamic(display, &debug_screen.vertices).unwrap();
        let debug_quad_index_buffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                                                     &debug_screen.indices)
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
                world: world_array,
                world_view_projection: array4x4(world_view_projection_matrix),
                position_texture: position_texture,
                normal_texture: normal_texture,
                diffuse_texture: diffuse_texture,
                light_is_distant: light.is_distant,
                light_direction: light.direction,
                light_position: light.position,
                light_attenuation: light.attenuation,
                light_color: light.color,
                light_radius: light.radius,
                viewPos: array4(camera_position),
            );

            match programs.get("light_shader") {
                Some(t) => {
                    light_buffer.draw(&quad_vertex_buffer, &quad_index_buffer, &t, &light_uniform, &light_params).unwrap();
                }
                None => ()
            }
        }
        //---------------------------DRAW-LIGHTS-END----------------------//

        //---------------------------SHADOW-START-------------------------//
        let w = 20.0;
        let depth_proj_matrix = cgmath::ortho(-w, w, -w, w, -100.0, 1024.0);
        let view_center = cgmath::Point3::new(0.0, 0.0, 0.0);
        let view_up = cgmath::Vector3::new(0.0, 1.0, 0.0);
        let light_pos = cgmath::Point3::new(-1.0, 0.5, -1.0);
        let light_dir = cgmath::Point3::new(0.0, 0.0, 0.0) - light_pos;
        let depth_view_matrix = cgmath::Matrix4::look_at(light_pos, view_center, view_up);

        {
            let depth_view_matrix = depth_view_matrix * world_matrix;
            let mut draw_params: glium::draw_parameters::DrawParameters = Default::default();
            draw_params.depth = glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            };

            draw_params.backface_culling = glium::BackfaceCullingMode::CullCounterClockwise;

            shadow_buffer.clear_color(1.0, 1.0, 1.0, 1.0);
            shadow_buffer.clear_depth(1.0);


            for model in &models {
                let model_borrowed = model.borrow();
                let model_matrix = model_borrowed.get_matrix();
                let matrix: [[f32; 4]; 4] = array4x4(model_matrix.invert().unwrap());

                let uniform = uniform! {
                    depth_mvp : array4x4(depth_proj_matrix * depth_view_matrix * model_matrix)
                };

                let buff = glium::VertexBuffer::new(display, &model.borrow().get_vertices()).unwrap();
                match programs.get("shadow_shader") {
                    Some(t) => {
                        shadow_buffer.draw(&buff, &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &t, &uniform, &draw_params).unwrap();
                    }
                    None => ()
                }
            }
        }

        //Shadow drawing
        {
            let mut draw_params : glium::draw_parameters::DrawParameters = Default::default();
            draw_params.depth = glium::Depth{
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            };
            draw_params.backface_culling = glium::BackfaceCullingMode::CullClockwise;
            draw_params.blend = glium::Blend::alpha_blending();


            shadow_render_buffer.clear_color_and_depth((0.0,0.0,0.0,0.0),1.0);

            let bias_matrix: cgmath::Matrix4<f32> = [
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 0.5, 0.0],
                [0.5, 0.5, 0.5, 1.0],
            ].into();


            for model in &models {
                let model_borrowed = model.borrow();
                let model_matrix = model_borrowed.get_matrix();

                let converted: [f32; 3] = Vector3::new(0.5, 0.7, 1.0).into();

                let model_uniform = uniform!(
                    depth_bias_mvp: array4x4(bias_matrix * depth_proj_matrix * depth_view_matrix * model_matrix),
                    u_worldViewProjection: array4x4(world_view_projection_matrix * model_matrix),
                    u_world: world_array,
                    tex: objects_textures,
                    reverse_light_direction: converted,
                    light_dir:array3(light_dir),
                     shadow_texture: glium::uniforms::Sampler::new(shadow_texture)
                         .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                         .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                         .depth_texture_comparison(Some(glium::uniforms::DepthTextureComparison::LessOrEqual))
                         .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp),
                );

                let buff = glium::VertexBuffer::new(display, &model_borrowed.get_vertices()).unwrap();

                match programs.get("shadow_render_shader") {
                    Some(t) => {
                        match shadow_render_buffer.draw(&buff, &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), &t, &model_uniform, &draw_params) {
                         Ok(t) => (),
                            Err(e) => println!("error {:#?}", e),
                        }
                    }
                    None => ()
                }
            }
        }


        //---------------------------SHADOW-END-------------------------//

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

            shadow_light_view_matrix: array4x4(depth_proj_matrix * depth_view_matrix),
            matrix: ortho_matrix,
            shadow_render_texture: shadow_render_texture,
             diffuse_texture: diffuse_texture,
             light_texture: light_texture,
             ui_texture: ui_texture,
             shadow_texture_debug: glium::uniforms::Sampler::new(shadow_texture)
                 .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                 .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
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

//        {
//            let uniforms = uniform! {
//                matrix: ortho_matrix,
//                 diffuse_texture: diffuse_texture,
//                 light_texture: light_texture,
//                 ui_texture: ui_texture,
//                 shadow_texture: glium::uniforms::Sampler::new(shadow_texture)
//                 .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
//                 .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
//                 .depth_texture_comparison(Some(glium::uniforms::DepthTextureComparison::LessOrEqual)),
//                 shadow_texture_debug: glium::uniforms::Sampler::new(shadow_texture)
//                 .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
//                 .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
//            };
//            let mut target = display.draw();
//
//            target.clear_color(0.0, 0.0, 1.0, 1.0);
//
//            match programs.get("screen_shader") {
//                Some(program) => {
//                    target.draw(&debug_quad_vertex_buffer, &debug_quad_index_buffer, program, &uniforms, &Default::default())
//                        .unwrap();
//                }
//                None => ()
//            }
//        }

        //-------------------DEFFERED-RENDERING-DRAW-END--------------------//
    }
}
