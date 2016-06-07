pub struct PhysicalBody {
    name: String,
    left_up_corner: [f32;2],
    right_down_corner: [f32;2],

}

impl PhysicalBody {
    pub fn generate_actors() -> Vec<Actor> {

    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn sould_generate_actors(){
        
    }
}
