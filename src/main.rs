#[macro_use]
extern crate glium;
extern crate image;

mod vertex;
use vertex::Vertex;
mod sprite;
use sprite::Sprite;
mod graphic_item;


use sprite::GraphicItem;
use sprite::ImageManager;
use std::io::Cursor;
use glium::{DisplayBuild, Surface};


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
    let display = glium::glutin::WindowBuilder::new()
                                .build_glium()
                                .unwrap();

    // let vert1 = Vertex { position: [-0.5, 0.5], normal: [0.0,0.0,-1.0]};
    // let vert2 = Vertex { position: [0.5, 0.5], normal: [0.0,0.0,-1.0]};
    // let vert3 = Vertex { position: [0.5, -0.5], normal: [0.0,0.0,-1.0]};
    // let vert4 = Vertex { position: [-0.5, -0.5], normal: [0.0,0.0,-1.0]};
    //
    // let vert5 = Vertex { position: [0.0, 1.0], normal: [0.0,0.0,-1.0]};
    // let vert6 = Vertex { position: [1.0, 1.0], normal: [0.0,0.0,-1.0]};
    // let vert7 = Vertex { position: [1.0, 0.0], normal: [0.0,0.0,-1.0]};
    // let vert8 = Vertex { position: [0.0, 0.0], normal: [0.0,0.0,-1.0]};


    let vert = vec![Sprite::new(0.0,0.0,[1.0,0.0,0.0,1.0]), Sprite::new(0.5,0.5,[0.0,1.0,0.0,1.0])];


    // let vertex_shader = vert[0].get_vertex_shader();
    // let fragment_shader = vert[0].get_fragment_shader();
    // let index_buffer = vec![0,1,2,0,2,3u16];
    // let (vertex_buffer,index_buffer) = generate_sprite(&display, &vert);



    loop{
        let mut target = display.draw();
        target.clear_color(0.0,0.0,1.0,1.0);

        // let ib_slice = index_buffer.slice(0..vert.len() * 6).unwrap();



        for v in &vert{

            let vertex_shader = v.get_vertex_shader();
            let fragment_shader = v.get_fragment_shader();
            let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();


            let vertex_buffer = glium::VertexBuffer::new(&display, &v.vertices).unwrap();
            let indices = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &v.indices).unwrap();

            let tex = v.set_image("../content/NatureForests.png").unwrap();
            let texture = glium::texture::Texture2d::new(&display, tex).unwrap();

            let uniforms = uniform! {
                matrix: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ],
                tex: &texture,
            };

            target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        }

        target.finish().unwrap();





        for ev in display.poll_events(){
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
