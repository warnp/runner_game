
#[derive(Copy, Clone, Debug)]
pub struct Vertex{
    pub position: [f32; 2],
    pub normal: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
    pub i_tex_id: u32,
}


implement_vertex!(Vertex, position, normal,color, tex_coords,i_tex_id);
