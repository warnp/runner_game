use engine::model::Model;
use engine::vertex;

extern crate glium;

#[derive(Clone, Debug)]
pub struct ObjectManager {}

impl ObjectManager {
    pub fn get_buffers(display: &glium::backend::glutin_backend::GlutinFacade, models: Vec<Model>) ->
    (glium::VertexBuffer<vertex::Vertex>, glium::IndexBuffer<u16>) {
        let mut vertice_array = Vec::new();
        for model in &models {
            for vertex in model.vertices.iter() {
                vertice_array.push(*vertex);
            }
        }

        let mut indice_array = Vec::new();
        let mut iterator: u16 = 0;
        for model in &models {
            let offset = model.indices.len() as u16;
            for index in model.clone().indices {
                indice_array.push(index + offset * iterator );
            }
            iterator = iterator + 1;
        }

        (glium::VertexBuffer::dynamic(display, &vertice_array).unwrap(),
         glium::index::IndexBuffer::new(display,
                                        glium::index::PrimitiveType::TrianglesList,
                                        &indice_array)
             .unwrap())
    }
}