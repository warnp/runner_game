use engine::generic_object::GenericObject;

#[derive(Clone,Debug)]
pub struct Actor<'a>{
    name: String,
    position: [f32; 2],
    image: &'a str,
    size: [f32; 2],
}

impl<'a> Actor<'a>{
    pub fn new(name: String, position: [f32; 2], image: &'a str, size: [f32; 2]) -> Actor<'a> {
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
    fn get_position(&self) -> (f32,f32,f32){
        (self.position[0],self.position[1],0.0)
    }
    fn get_name(&self) -> String {
        (&self.name).to_string()
    }
    fn get_description(&self) -> String {
        "This is an actor".to_string()
    }
}

// impl<'a> Actor<'a> {
//     pub fn update_position(&self, new_position: [f32; 2]) ->[f32;2]{
//
//     }
// }

#[cfg(test)]
mod tests{
    use super::*;

    fn should_update_position(){
        let actor = Actor::new("an_actor", [0.0,0.0],"an_image", [1.0,1.0]);

        actor.update_position([1.0,0.0]);

        assert_eq!(actor.get_position(),[1.0,0.0]);
    }
}
