use engine::generic_object::GenericObject;

#[derive(Debug)]
pub struct Actor<'a>{
    name: &'a str,
    position: [f32; 2],
    image: &'a str,
    size: [f32; 2],
}

impl<'a> Actor<'a>{
    pub fn new(name: &'a str, position: [f32; 2], image: &'a str, size: [f32; 2]) -> Actor<'a> {
        Actor{
            name: name,
            position: position,
            image: image,
            size: size,
        }
    }
}
impl<'a> GenericObject for Actor<'a> {
    fn get_type(&self) -> String {
        "Sprite".to_string()
    }
    fn get_position(&self) -> (f64,f64,f64){
        (0.0,0.0,0.0)
    }
    fn get_name(&self) -> String {
        "Actor".to_string()
    }
}
