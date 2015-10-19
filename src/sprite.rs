use vertex;

use glium::backend::Facade;

use std::io::Cursor;
extern crate image;
extern crate glium;

use glium::{DisplayBuild, Surface};


pub trait GraphicItem {
    fn get_position(&self) -> [f32; 2];
    fn get_vertex_shader(&self) -> &str;
    fn get_fragment_shader(&self) -> &str;
    // fn get_vertex_buffer<F>(&self,facade: &F,display: &glium::glutin::WindowBuilder) -> glium::VertexBuffer<vertex::Vertex> where F: Facade;
    // fn get_index_buffer(&self,display: &glium::glutin::WindowBuilder) -> glium::IndexBuffer<u32>;
}

pub trait ImageManager {
    fn set_image(&self, display: &glium::backend::glutin_backend::GlutinFacade, image_path: &str) -> image::ImageResult<image::DynamicImage>;
}

#[derive(Copy, Clone, Debug)]
pub struct Sprite{
    pub vertices: [vertex::Vertex; 4],
    pub indices: [u16; 6],
    // pub transform: [[f32; 4]; 4],
    // pub display: &glium::glutin::WindowBuilder,
}

impl Sprite {
    pub fn new(x: f32, y: f32,color: [f32; 4]) -> Sprite {

        Sprite {
            vertices : [   vertex::Vertex { position: [-0.1 + x, 0.1 + y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [0.0,0.0]},
                            vertex::Vertex { position: [0.1 + x, 0.1 + y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [0.0,1.0]},
                            vertex::Vertex { position: [0.1 + x, -0.1 + y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [1.0,0.0]},
                            vertex::Vertex { position: [-0.1 + x, -0.1 + y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [1.0,1.0]}],
            indices : [0,1,2,0,2,3],
            }

    }
}

impl ImageManager for Sprite {
    fn set_image(&self, display: &glium::backend::glutin_backend::GlutinFacade,image_path: &str) ->image::ImageResult<image::DynamicImage>{


        image::load(Cursor::new(&include_bytes!("../content/NatureForests.png")[..]),
            image::PNG).unwrap()

        // Box::new(glium::Texture2d::new(display, &image_to_load).unwrap())
    }
}


impl GraphicItem for Sprite {

    fn get_position(&self) -> [f32; 2] {

        let x = (self.vertices[0].position[0] + self.vertices[1].position[0] + self.vertices[2].position[0] + self.vertices[3].position[0]) as f32 / 4.0;
        let y = (self.vertices[0].position[1] + self.vertices[1].position[1] + self.vertices[2].position[1] + self.vertices[3].position[1]) as f32;
        [x,y]
    }

    fn get_vertex_shader(&self) -> &str{
        r#"
        #version 140

        in vec2 position;
        in vec3 normal;
        in vec4 color;
        in vec2 tex_coords;

        out vec4 colorV;
        out vec3 v_normal;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main(){
            // colorV = color;
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0,1.0);
        }
        "#
    }

    fn get_fragment_shader(&self) -> &str{
        r#"
        #version 140

        in vec4 colorV;
        in vec2 v_tex_coords;

        out vec4 color;

        uniform sampler2D tex;

        void main(){
            color = texture(tex, v_tex_coords);
        }
        "#
    }

    // fn get_vertex_buffer<F>(&self,facade: &F, display: &glium::glutin::WindowBuilder) -> glium::VertexBuffer<vertex::Vertex>
    //         where F : Facade
    // {
    //     glium::VertexBuffer::new(display, &self.vertices).unwrap()
    // }


    //
    // fn get_index_buffer(&self, display: &glium::glutin::WindowBuilder) -> glium::IndexBuffer<u32>{
    //     glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
    // }
}


// impl fmt::Debug for Sprite {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Hi")
//     }
// }







#[cfg(test)]
mod tests {
    use super::*;
    use vertex;
    use sprite;
    // use sprite::Position;

    #[test]
    fn should_calculate_center_of_sprite_position(){
        //Given
        let vert1 = vertex::Vertex { position: [-0.5, 0.5]};
        let vert2 = vertex::Vertex { position: [0.5, 0.5]};
        let vert3 = vertex::Vertex { position: [0.5, -0.5]};
        let vert4 = vertex::Vertex { position: [-0.5, -0.5]};
        let sprite_test = sprite::Sprite {vertices: [vert1, vert2, vert3, vert4] };
        //when
        let position_result = sprite_test.get_position();

        //then
        assert_eq!(position_result, [0.0,0.0]);
    }


}
