
#[derive(Copy, Clone, Debug)]
pub struct Vertex{
    pub position: [f32; 2],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

// impl fmt::Debug for Vertex {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Hi")
//     }
// }

implement_vertex!(Vertex, position, normal,color, tex_coords);
