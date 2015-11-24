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

mod collision;
use collision::CollisionMesh;

mod text_writer;
use text_writer::TextWriter;

use glium::{DisplayBuild, Surface};


fn jump_function(sp: &mut [vertex::Vertex], jump: &mut bool, touch_ground: &mut bool, jump_height: f32, time_between: f32, index: u32, sprite_manager: &SpriteManager) {

    if *jump && *touch_ground {

        *touch_ground = false;

        sp[0].position[1] = jump_height + sp[0].position[1];
        sp[1].position[1] = jump_height + sp[1].position[1];
        sp[2].position[1] = jump_height + sp[2].position[1];
        sp[3].position[1] = jump_height + sp[3].position[1];

    } else {


        *touch_ground = false;
        *jump = false;

        // sp[0].position[1] = positions_buffer[0].1 .1;
        // sp[1].position[1] = positions_buffer[0].2 .1;
        // sp[2].position[1] = positions_buffer[0].3 .1;
        // sp[3].position[1] = positions_buffer[0].4 .1;
        //
        // positions_buffer[0].1 .1 = sp[0].position[1] - 0.3 * time_between;
        // positions_buffer[0].2 .1 = sp[1].position[1] - 0.3 * time_between;
        // positions_buffer[0].3 .1 = sp[2].position[1] - 0.3 * time_between;
        // positions_buffer[0].4 .1 = sp[3].position[1] - 0.3 * time_between;

        //=======================================


        // sp[0].position[1] = sp[0].position[1] - 0.3 * time_between;
        // sp[1].position[1] = sp[1].position[1] - 0.3 * time_between;
        // sp[2].position[1] = sp[2].position[1] - 0.3 * time_between;
        // sp[3].position[1] = sp[3].position[1] - 0.3 * time_between;

        let sprite = sprite_manager.move_sprite("hero", 0.0, -0.3 * time_between);

        // let sprite = sprite_manager.get_sprites_coordinate("hero");
        // println!("{:?}", sprite.vertices);
        sp[0].position[1] = sprite.vertices[0].position[1];
        sp[1].position[1] = sprite.vertices[1].position[1];
        sp[2].position[1] = sprite.vertices[2].position[1];
        sp[3].position[1] = sprite.vertices[3].position[1];

    }
}

fn move_to_left(sp: &mut [vertex::Vertex], time_between: f32){
    sp[0].position[0] = sp[0].position[0] - 0.5 * time_between;
    sp[1].position[0] = sp[1].position[0] - 0.5 * time_between;
    sp[2].position[0] = sp[2].position[0] - 0.5 * time_between;
    sp[3].position[0] = sp[3].position[0] - 0.5 * time_between;
}



fn main() {


    let screen_height = 768.0;
    let screen_width = 1024.0;
    let mut show_fps = false;
    let display = glium::glutin::WindowBuilder::new()
                                .with_vsync()
                                .with_dimensions(screen_width as u32,screen_height as u32)
                                .build_glium()
                                .unwrap();



    let mut vert = vec![Sprite::new("hero",0.0,0.0,[1.0,0.0,0.0,1.0],0,(0.10,0.10)),
                    Sprite::new("mover0",0.5,0.0,[1.0,0.0,0.0,1.0],1,(2.0,1.0))];



    let mut shaders = shader_manager::Shaders::new(vec![&include_bytes!("../content/VFKM2.png")[..],&include_bytes!("../content/11532.png")[..]]);
    shaders.compile_shaders(&display);

    let program = shaders.get_compiled_shader("simple_shader");


    let sprite_manager = SpriteManager::new(vert);
    let texture = shaders.get_texture_array(&display);

    let mut vertex_buffer = sprite_manager.get_vertex_buffer(&display);
    let mut indices = sprite_manager.get_index_buffer(&display);

    let mut buffers = (vertex_buffer, indices);

    let mut go_up = false;
    let jump_height = 0.5;
    let mut jump = false;
    let mut touch_ground = false;


    let mut t : f32 = 0.0;
    let mut old_time = 0.0;
    let mut horizontal_position = 0.0;
    let text_manager = text_writer::TextWriter::new(0,(256,256),(16,16));

    loop{
        let mut target = display.draw();


        target.clear_color(0.0,0.0,1.0,1.0);

        t = t + 1.0;

        let time = time::precise_time_ns() as f32;
        let mut time_between = 0.0;
        if t > 60.0{
            time_between = time/1000000000.0 - old_time/1000000000.0;
        }

        let fps = 1.0/(time_between );
        if show_fps {
            println!("FPS : {}", fps);
        }
        old_time = time;

        {
            let mut mapping = buffers.0.map();
            let mut index = 0;
            for sp in mapping.chunks_mut(4){

                if index == 0 {
                    jump_function(sp, &mut jump, &mut touch_ground, jump_height, time_between, 0, &sprite_manager);

                    let coord = text_manager.get_coordinates("toto");

                    // println!("{:?}", coord);
                    sp[0].tex_coords[0] = (coord[0].0).0;
                    sp[1].tex_coords[0] = (coord[0].1).0;
                    sp[2].tex_coords[0] = (coord[0].2).0;
                    sp[3].tex_coords[0] = (coord[0].3).0;

                    sp[0].tex_coords[1] = (coord[0].0).1;
                    sp[1].tex_coords[1] = (coord[0].1).1;
                    sp[2].tex_coords[1] = (coord[0].2).1;
                    sp[3].tex_coords[1] = (coord[0].3).1;


                }
                else {
                    move_to_left(sp, time_between);
                }


                index = index + 1;
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

        target.draw(&buffers.0, &buffers.1, &program, &uniforms,
                &Default::default()).unwrap();

        target.finish().unwrap();


        for ev in display.poll_events(){
            // println!("{:?}", ev);
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Space)) => jump = true,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Space)) => horizontal_position = 0.0,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released,_,Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,_,Some(glium::glutin::VirtualKeyCode::A)) => buffers = sprite_manager.add_sprite(Sprite::new("mover1",0.5,0.5,[1.0,0.0,0.0,1.0],1,(2.0,1.0)), &display),
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,_,Some(glium::glutin::VirtualKeyCode::D)) => buffers = sprite_manager.delete_sprite("mover1", &display),
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released,_,Some(_)) => println!("BONJOUR" ),
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
