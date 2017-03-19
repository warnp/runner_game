use engine::generic_object::GenericObject;
use engine::generic_object_type::GenericObjectType;

#[derive(Clone, Debug)]
pub struct Actor {
    name: String,
    position: [f32; 2],
    image: i32,
    size: [f32; 2],
    texture_coordinates: ((f32, f32), (f32, f32),(f32, f32), (f32, f32)),
}

impl Actor {
    pub fn new(name: String, position: [f32; 2], image: i32, size: [f32; 2], texture_coordinates: ((f32, f32), (f32, f32),(f32, f32), (f32, f32))) -> Actor {
        Actor {
            name: name,
            position: position,
            image: image,
            size: size,
            texture_coordinates: texture_coordinates,
        }
    }
}

impl GenericObject for Actor {
    fn get_type(&self) -> GenericObjectType {
        GenericObjectType::Sprite
    }
    fn get_position(&self) -> (f32, f32, f32) {
        (self.position[0], self.position[1], 0.0)
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
    fn get_size(&self) -> (f32, f32, f32) {
        (self.size[0], self.size[1], 0.0)
    }
    fn get_texture_coordinates(&self) -> ((f32,f32),(f32,f32),(f32,f32),(f32,f32)) {
        self.texture_coordinates
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::generic_object_type::GenericObjectType;


    fn get_actor() -> Actor {
        Actor::new("an_actor".to_string(), [0.0, 0.0], 0, [1.0, 1.0], ((), ()))
    }

    #[test]
    fn should_get_position() {
        let actor = get_actor();

        assert_eq!(actor.get_position(), (0.0,0.0,0.0));
    }

    #[test]
    fn should_get_name() {
        let actor = get_actor();

        assert_eq!(actor.get_name(), "an_actor".to_string());
    }
    #[test]
    fn should_get_texture_id() {
        let actor = get_actor();

        assert_eq!(actor.get_texture_id(), 0);
    }
    #[test]
    fn should_get_type() {
        let actor = get_actor();

        assert_eq!(actor.get_type(), GenericObjectType::Sprite);
    }
}
