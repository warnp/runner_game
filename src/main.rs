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

fn jump_function(sp: &mut [vertex::Vertex], jump: &mut bool, touch_ground: &mut bool, jump_height: f32, time_between: f32) {

    if *jump && *touch_ground {

        *touch_ground = false;
        sp[0].position[1] = jump_height + sp[0].position[1];
        sp[1].position[1] = jump_height + sp[1].position[1];
        sp[2].position[1] = jump_height + sp[2].position[1];
        sp[3].position[1] = jump_height + sp[3].position[1];

    } else {
        if sp[2].position[1] > -0.5 {
            *touch_ground = false;
            *jump = false;
            sp[0].position[1] = sp[0].position[1] - 0.3 * time_between;
            sp[1].position[1] = sp[1].position[1] - 0.3 * time_between;
            sp[2].position[1] = sp[2].position[1] - 0.3 * time_between;
            sp[3].position[1] = sp[3].position[1] - 0.3 * time_between;
        } else {
            *touch_ground = true;

        }
    }
}

fn move_to_left(sp: &mut [vertex::Vertex], time_between: f32){
    sp[0].position[0] = sp[0].position[0] - 0.5 * time_between;
    sp[1].position[0] = sp[1].position[0] - 0.5 * time_between;
    sp[2].position[0] = sp[2].position[0] - 0.5 * time_between;
    sp[3].position[0] = sp[3].position[0] - 0.5 * time_between;
}

struct CollisionMesh {
    aa: [f32; 2],
    bb: [f32; 2],
}

impl CollisionMesh {
    fn detect_collide(&self,aa: [f32; 2], bb: [f32; 2]) -> bool {
        if self.aa[0] <= bb[0] &&
            self.aa[1] >= bb[1] &&
            self.bb[0] >= aa[0] &&
            self.bb[1] <= aa[1] {
                true
            }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_collide(){
        let collider :CollisionMesh {
            aa: [-0.5,0.1],
            bb: [1.0,-1.0],
        };

        assert!(collider.detect_collide([-1.0, 1.0], [0.0,0.0]));
    }
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



    let vert = vec![Sprite::new(0.0,0.0,[1.0,0.0,0.0,1.0],0),
                    Sprite::new(0.5,0.0,[1.0,0.0,0.0,1.0],1)];


    let mut shaders = shader_manager::Shaders::new(vec![&include_bytes!("../content/VFKM2.png")[..],&include_bytes!("../content/11532.png")[..]]);
    shaders.compile_shaders(&display);

    let program = shaders.get_compiled_shader("simple_shader");


    let sprite_manager = SpriteManager::new(vert);
    let texture = shaders.get_texture_array(&display);

    let mut vertex_buffer = sprite_manager.get_vertex_buffer(&display);
    let indices = sprite_manager.get_index_buffer(&display).unwrap();



    let mut go_up = false;
    let jump_height = 0.5;
    let mut jump = false;
    let mut touch_ground = false;


    let mut t : f32 = 0.0;
    let mut old_time = 0.0;
    let mut horizontal_position = 0.0;
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
            let mut mapping = vertex_buffer.map();
            let mut index = 0;
            for sp in mapping.chunks_mut(4){
                // sp[0].position[0] = 0.01 * time_between + sp[0].position[0];
                // sp[1].position[0] = 0.01 * time_between + sp[1].position[0];
                // sp[2].position[0] = 0.01 * time_between + sp[2].position[0];
                // sp[3].position[0] = 0.01 * time_between + sp[3].position[0];

                if index == 0 {
                    jump_function(sp, &mut jump, &mut touch_ground, jump_height, time_between);

                    if t % 10.0 == 0.0 {
                        sp[0].tex_coords[0] = 0.0625 + sp[0].tex_coords[0];
                        sp[1].tex_coords[0] = 0.0625 + sp[1].tex_coords[0];
                        sp[2].tex_coords[0] = 0.0625 + sp[2].tex_coords[0];
                        sp[3].tex_coords[0] = 0.0625 + sp[3].tex_coords[0];
                    }
                }
                else {
                    move_to_left(sp, time_between);
                }

                //
                // if t >= 60.0 && t % 10.0 == 0.0 {
                //     if t % 5.0 != 0.0 {
                //         sp[0].tex_coords[0] = 0.0977 + sp[0].tex_coords[0];
                //         sp[1].tex_coords[0] = 0.0977 + sp[1].tex_coords[0];
                //         sp[2].tex_coords[0] = 0.0977 + sp[2].tex_coords[0];
                //         sp[3].tex_coords[0] = 0.0977 + sp[3].tex_coords[0];
                //     } else {
                //         sp[0].tex_coords[0] = (-0.0977) * 5.0 + sp[0].tex_coords[0];
                //         sp[1].tex_coords[0] = (-0.0977) * 5.0 + sp[1].tex_coords[0];
                //         sp[2].tex_coords[0] = (-0.0977) * 5.0 + sp[2].tex_coords[0];
                //         sp[3].tex_coords[0] = (-0.0977) * 5.0 + sp[3].tex_coords[0];
                //     }
                // }
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

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                &Default::default()).unwrap();

        target.finish().unwrap();


        for ev in display.poll_events(){
            println!("{:?}", ev);
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Space)) => jump = true,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released, _, Some(glium::glutin::VirtualKeyCode::Space)) => horizontal_position = 0.0,
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released,_,Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
