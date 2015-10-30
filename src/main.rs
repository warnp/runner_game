#[macro_use]
extern crate glium;
extern crate time;

mod vertex;
use vertex::Vertex;
mod sprite;
use sprite::Sprite;
mod graphic_item;
mod shader_manager;
use shader_manager::{Shaders, ShaderCouple};
mod sprite_manager;
use sprite_manager::SpriteManager;

use sprite::GraphicItem;
use glium::{DisplayBuild, Surface};

#[warn(dead_code)]
fn generate_sprite(display: &glium::backend::glutin_backend::GlutinFacade, sprites: &Vec<Sprite>) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>){

    let mut vb : glium::VertexBuffer<Vertex> = glium::VertexBuffer::empty_dynamic(display, sprites.len() * 4).unwrap();
    let mut ib_data = Vec::with_capacity(sprites.len() * 6);

    for num in 0..vb.map().chunks_mut(4).enumerate().len() {
        let num = num as u16;
        ib_data.push(num * 4);
        ib_data.push(num * 4 + 1);
        ib_data.push(num * 4 + 2);
        ib_data.push(num * 4 + 1);
        ib_data.push(num * 4 + 3);
        ib_data.push(num * 4 + 2);
    }

    (vb, glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &ib_data).unwrap())

}

fn main() {
    let screen_height = 600.0;
    let screen_width = 800.0;
    let mut show_fps = false;
    let display = glium::glutin::WindowBuilder::new()
                                //  .with_vsync()
                                .with_dimensions(screen_width as u32,screen_height as u32)
                                .build_glium()
                                .unwrap();



    let vert = vec![Sprite::new(0.0,0.0,[1.0,0.0,0.0,1.0],0),
                    Sprite::new(0.5,0.0,[1.0,0.0,0.0,1.0],1)];

    // println!("{:?}", vert);

    let mut shaders = shader_manager::Shaders::new(vec![&include_bytes!("../content/NatureForests.png")[..],&include_bytes!("../content/11532.png")[..]]);
    shaders.compile_shaders(&display);

    let program = shaders.get_compiled_shader("simple_shader");


    let sprite_manager = SpriteManager::new(vert);
    let texture = shaders.get_texture_array(&display);

    let mut vertex_buffer = sprite_manager.get_vertex_buffer(&display);
    let indices = sprite_manager.get_index_buffer(&display).unwrap();
    // let img = v.set_image().unwrap();


    let mut t : f32 = 0.0;
    // let mut time : f32 = time::precise_time_ns() as f32;
    let mut old_time = 0.0;
    loop{
        let mut target = display.draw();

        target.clear_color(0.0,0.0,1.0,1.0);

        t = t + 1.0;

        let time = time::precise_time_ns() as f32;
        let mut time_between = 0.0;
        if t > 60.0{
            time_between = time/1000000000.0 - old_time/1000000000.0;
        }
        // println!("{:?}", time_between);
        let fps = 1.0/(time_between );
        if show_fps {
            println!("FPS : {}", fps);
        }
        old_time = time;


        {
            let mut mapping = vertex_buffer.map();

            for sp in mapping.chunks_mut(4){
                sp[0].position[0] = 0.01 * time_between + sp[0].position[0];
                sp[1].position[0] = 0.01 * time_between + sp[1].position[0];
                sp[2].position[0] = 0.01 * time_between + sp[2].position[0];
                sp[3].position[0] = 0.01 * time_between + sp[3].position[0];

                sp[0].i_tex_id = 1;
                sp[1].i_tex_id = 1;
                sp[2].i_tex_id = 1;
                sp[3].i_tex_id = 1;


            }


        }




        let uniforms = uniform! {
            matrix: [
                [screen_height/screen_width, 0.0 , 0.0 , 0.0],
                [0.0                       , 1.0 , 0.0 , 0.0],
                [0.0                       , 0.0 , 1.0 , 0.0],
                [0.0                       , 0.0 , 0.0 , 1.0f32],
            ],
            tex: &texture,
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                &Default::default()).unwrap();

        target.finish().unwrap();





        for ev in display.poll_events(){
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
