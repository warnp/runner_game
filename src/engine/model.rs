use engine::vertex::Vertex;
use engine::graphic_item::GraphicItem;

#[derive(Clone, Debug)]
pub struct Model {
    pub vertices: [Vertex; 6],
    pub indices: [u16; 6],
    pub name: String
}

impl Model {
    pub fn new(name: String,
                x:f32,
                y:f32,
                z:f32,
                color: [f32; 4],
                size: (f23, f32, f32)) -> Model {
        Model {
            vertices: [
                Vertex {
                    position: [-0.5 * size.0 + x, 0.5 * size.1 + y,0.5* size.2 - z],
                    normal: [0.0, 0.0, -1.0],
                    color: color,
                    tex_coords: [tex_coord.0 .0, tex_coord.0 .1],
                    i_tex_id: tex_id,
                },
                Vertex {
                    position: [0.5 * size.0 + x, 0.5 * size.1 + y,0.5* size.2 - z],
                    normal: [0.0, 0.0, -1.0],
                    color: color,
                    tex_coords: [tex_coord.1 .0, tex_coord.1 .1],
                    i_tex_id: tex_id,
                },
                Vertex {
                    position: [0.5 * size.0 + x, -0.5 * size.1 + y,0.5* size.2 - z],
                    normal: [0.0, 0.0, -1.0],
                    color: color,
                    tex_coords: [tex_coord.2 .0, tex_coord.2 .1],
                    i_tex_id: tex_id,
                },
                Vertex {
                    position: [-0.5 * size.0 + x, -0.5 * size.1 + y,0.5* size.2 - z],
                    normal: [0.0, 0.0, -1.0],
                    color: color,
                    tex_coords: [tex_coord.3 .0, tex_coord.3 .1],
                    i_tex_id: tex_id,
                },
                Vertex {
                    position: [-0.5 * size.0 + x, 0.5 * size.1 + y,0.5* size.2 + z],
                    normal: [0.0, 0.0, -1.0],
                    color: color,
                    tex_coords: [tex_coord.0 .0, tex_coord.0 .1],
                    i_tex_id: tex_id,
                },
                Vertex {
                    position: [0.5 * size.0 + x, -0.5 * size.1 + y,0.5* size.2 + z],
                    normal: [0.0, 0.0, -1.0],
                    color: color,
                    tex_coords: [tex_coord.2 .0, tex_coord.2 .1],
                    i_tex_id: tex_id,
                },
                Vertex {
                    position: [-0.5 * size.0 + x, -0.5 * size.1 + y,0.5* size.2 + z],
                    normal: [0.0, 0.0, -1.0],
                    color: color,
                    tex_coords: [tex_coord.3 .0, tex_coord.3 .1],
                    i_tex_id: tex_id,
                }],
            indices: [0,1,2,
                      0,2,3,
                      1,4,5,
                      1,5,2,
                      4,6,7,
                      4,7,5,
                      6,0,3,
                      6,3,7],
            name: name
        }
    }
}