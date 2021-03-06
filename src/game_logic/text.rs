use engine::generic_object::GenericObject;
use engine::generic_object_type::GenericSpriteType;

pub struct Text {
    name: String,
    text_content: String,
    position: [f32; 2],
    order: u8,
}

impl Text {
    pub fn new(name: String, position: [f32; 2], order: u8, content: String) -> Text {
        Text {
            name: name,
            text_content: content,
            position: position,
            order: order,
        }
    }
}

impl GenericObject for Text {
    fn get_type(&self) -> GenericSpriteType {
        GenericSpriteType::TEXT
    }
    fn get_position(&self) -> (f32, f32, f32) {
        (self.position[0], self.position[1], 0.0)
    }
    fn get_name(&self) -> String {
        (&self.name).to_string()
    }
    fn get_description(&self) -> String {
        (&self.text_content).to_string()
    }
    fn get_texture_id(&self) -> i32 {
        0 //For the moment it's truly 0, I think...
    }
    fn get_size(&self) -> (f32, f32, f32) {
        (0.0, 0.0, 0.0)
    }
    fn get_texture_coordinates(&self) -> ((f32, f32), (f32, f32), (f32, f32), (f32, f32)) {
        ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0))
    }
    fn get_order(&self) -> u8 {
        self.order
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::generic_object::GenericObject;

    #[ignore]
    #[test]
    fn should_set_text() {
        let text = Text::new("un text".to_string(), [0.0, 0.0], 0, "Toto".to_string());

        // assert_eq!(text.get_description(), "Toto".to_string());
    }
}
