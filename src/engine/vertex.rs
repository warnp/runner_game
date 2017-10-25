#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    //    pub normal: [f32; 3],
    //    pub color: [f32; 4],
    //    pub tex_coords: [f32; 2],
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

implement_vertex!(Normal, normal);
implement_vertex!(Vertex, position);
implement_vertex!(TexCoords, tex_coords);
