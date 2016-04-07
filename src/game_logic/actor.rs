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

}
