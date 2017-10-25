use engine::vertex::Vertex;
use engine::graphic_item::GraphicItem;
use std::u16;

#[derive(Clone, Debug)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
//    pub normals: Vec<f32>,
    pub name: String
}

impl Model {

    pub fn new(name: String,
               x: f32,
               y: f32,
               z: f32,
               color: [f32; 4],
               size: (f32, f32, f32)) -> Model {
        Model {
            vertices: vec![
                Vertex {
                    position: (1.0, 1.0, -1.0),
                    //0
//                    normal: [0.0, 1.0, 0.0],
//                    color: color,
//                    tex_coords: [0.0, 1.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (-1.0, 1.0, -1.0),
                    //1
//                    normal: [0.0, 1.0, 0.0],
//                    color: color,
//                    tex_coords: [1.0, 1.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (1.0, 1.0, 1.0),
                    //2
//                    normal: [0.0, 1.0, 0.0],
//                    color: color,
//                    tex_coords: [1.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (-1.0, 1.0, 1.0),
                    //3
//                    normal: [0.0, 1.0, 0.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (-1.0, -1.0, 1.0),
                    //4
//                    normal: [0.0, -1.0, 0.0],
//                    color: color,
//                    tex_coords: [0.0, 1.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (-1.0, 1.0, -1.0),
                    //5
//                    normal: [0.0, 1.0, 0.0],
//                    color: color,
//                    tex_coords: [1.0, 1.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (-1.0, -1.0, -1.0),
                    //6
//                    normal: [0.0, 0.0, -1.0],
//                    color: color,
//                    tex_coords: [1.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (1.0, 1.0, -1.0),
                    //7
//                    normal: [0.0, 0.0, -1.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (1.0, -1.0, -1.0),
                    //7
//                    normal: [0.0, 0.0, -1.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (1.0, 1.0, 1.0),
                    //7
//                    normal: [0.0, 0.0, 1.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (1.0, -1.0, 1.0),
                    //7
//                    normal: [0.0, 0.0, 1.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (-1.0, -1.0, 1.0),
                    //7
//                    normal: [0.0, 0.0, 1.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (1.0, -1.0, -1.0),
                    //7
//                    normal: [0.0, 0.0, 1.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                },
                Vertex {
                    position: (-1.0, -1.0, -1.0),
                    //7
//                    normal: [0.0, 0.0, 1.0],
//                    color: color,
//                    tex_coords: [0.0, 0.0],
//                    i_tex_id: 0,
                }
            ],
            indices: vec![ 0, 1, 2, 0, 2, 3,
                           4, 5, 6, 4, 6, 7,
                           3, 2, 5, 3, 5, 4,
                           2, 1, 6, 2, 6, 5,
                           1, 7, 6, 1, 0, 7,
                           0, 3, 4, 0, 4, 7u16
            ],
            name: name
        }
    }
}