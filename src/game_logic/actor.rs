use engine::generic_object::GenericObject;
use engine::generic_object_type::GenericObjectType;

#[derive(Clone,Debug)]
pub struct Actor{
    name: String,
    position: [f32; 2],
    image: i32,
    size: [f32; 2],
}

impl Actor{
    pub fn new(name: String, position: [f32; 2], image: i32, size: [f32; 2]) -> Actor {
        Actor{
            name: name,
            position: position,
            image: image,
            size: size,
        }
    }
}

impl GenericObject for Actor {
    fn get_type(&self) -> GenericObjectType {
        GenericObjectType::Sprite
    }
    fn get_position(&self) -> (f32,f32,f32) {
        (self.position[0],self.position[1],0.0)
    }
    fn get_name(&self) -> String {
        (&self.name).to_string()
    }
    fn get_description(&self) -> String {
        "This is an actor".to_string()
    }
    fn get_texture_id(&self) -> i32 {
        self.image
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    #[ignore]
    fn should_update_position(){
        let actor = Actor::new("an_actor".to_string(), [0.0,0.0],0, [1.0,1.0]);

        // actor.update_position([1.0,0.0]);

        // assert_eq!(actor.get_position(),[1.0,0.0]);
    }

    
}
