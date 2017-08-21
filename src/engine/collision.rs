
pub trait CollisionMesh {
    fn detect_collide(&self, aa: [f32; 2], bb: [f32; 2]) -> bool;
    fn get_aa_bb(&self) -> ([f32; 3], [f32; 3]);
}
