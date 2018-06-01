#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub normal: (f32, f32, f32),
    //    pub color: [f32; 4],
    pub tex_coords: (f32, f32),
    //    pub i_tex_id: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}

#[derive(Copy, Clone, Debug)]
pub struct TexCoords {
    pub tex_coords: (f32, f32),
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        if self.position.0 == other.position.0 && self.position.1 == other.position.1 && self.position.2 == other.position.2 {
            return true;
        }
        false
    }
}

impl Eq for Vertex {}

implement_vertex!(Normal, normal);
implement_vertex!(Vertex, position, normal, tex_coords);
implement_vertex!(TexCoords, tex_coords);
