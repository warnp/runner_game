use vertex;

extern crate glium;



pub trait GraphicItem {
    fn get_position(&self) -> [f32; 2];
    // fn get_vertex_shader(&self) -> &str;
    // fn get_fragment_shader(&self) -> &str;

    // fn get_texture(&self, display: &glium::backend::glutin_backend::GlutinFacade) -> Result<glium::texture::texture2d::Texture2d, glium::texture::TextureCreationError>;
}



#[derive(Copy, Clone, Debug)]
pub struct Sprite{
    pub vertices: [vertex::Vertex; 4],
    pub indices: [u16; 6],
    // pub transform: [[f32; 4]; 4],
    // pub display: &glium::glutin::WindowBuilder,
}

impl Sprite {
    pub fn new(x: f32, y: f32,color: [f32; 4],  tex_id: u32, size: (f32,f32)) -> Sprite {

        Sprite {
            vertices : [vertex::Vertex { position: [-0.1 * size.0 + x, 0.1 * size.1+ y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [0.0,1.0],i_tex_id: tex_id},
                        vertex::Vertex { position: [0.1* size.0 + x, 0.1 * size.1+ y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [1.0,1.0],i_tex_id: tex_id},
                        vertex::Vertex { position: [0.1 * size.0+ x, -0.1 * size.1+ y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [1.0,0.0],i_tex_id: tex_id},
                        vertex::Vertex { position: [-0.1 * size.0+ x, -0.1 * size.1+ y], normal: [0.0,0.0,-1.0], color: color, tex_coords: [0.0,0.0],i_tex_id: tex_id}],
            indices : [0,1,2,0,2,3],
            // transform: transform,

            }

    }
}


impl GraphicItem for Sprite {

    fn get_position(&self) -> [f32; 2] {

        let x = (self.vertices[0].position[0] + self.vertices[1].position[0] + self.vertices[2].position[0] + self.vertices[3].position[0]) as f32 / 4.0;
        let y = (self.vertices[0].position[1] + self.vertices[1].position[1] + self.vertices[2].position[1] + self.vertices[3].position[1]) as f32;
        [x,y]
    }



}


#[cfg(test)]
mod tests {
    use super::*;
    use vertex;
    use sprite;
    // use sprite::Position;

    //Not up to date
    // #[test]
    // fn should_calculate_center_of_sprite_position(){
    //     //Given
    //     let vert1 = vertex::Vertex { position: [-0.5, 0.5]};
    //     let vert2 = vertex::Vertex { position: [0.5, 0.5]};
    //     let vert3 = vertex::Vertex { position: [0.5, -0.5]};
    //     let vert4 = vertex::Vertex { position: [-0.5, -0.5]};
    //     let sprite_test = sprite::Sprite {vertices: [vert1, vert2, vert3, vert4] };
    //     //when
    //     let position_result = sprite_test.get_position();
    //
    //     //then
    //     assert_eq!(position_result, [0.0,0.0]);
    // }


}
