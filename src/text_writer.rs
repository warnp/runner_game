use vertex::Vertex;
use sprite::Sprite;

pub struct TextWriter {
    pub image_index : u16,
    pub image_size : (u16,u16),
    pub character_size : (u16,u16),
}

impl TextWriter {
    pub fn new(image_index: u16, image_size: (u16,u16), character_size:(u16,u16)) -> TextWriter {
        TextWriter {
            image_index: image_index,
            image_size: image_size,
            character_size: character_size,
        }
    }

    pub fn get_coordinates(&self, &str) -> ((u16,u16),(),(),()) {

    }
}



#[cfg(test)]
mod tests{
    use super::*;
    use glium::backend::Facade;
    use glium::{DisplayBuild, Surface};

    extern crate glium;
    //This test assume your bmpfont map is 256*256 with char 16*16
    #[test]
    fn should_set_charmap(){
        let writer = TextWriter::new(0, (256,256), (16,16));

        assert!(writer.image_index == 0);
        assert!(writer.image_size == (256,256));
        assert!(writer.character_size == (16,16));

    }

    #[test]
    fn should_give_characters_coordinate(){
        let writer = TextWriter::new(0, (256,256), (16,16));

        let coordinates = writer.get_coordinates("Blop");

        //TODO A modifier les coordonées !!!
        assert!(coordinates[0] == ((2.0,4.0),(3.0,4.0),(3.0,5.0),(2.0,5.0));
        // let display = glium::glutin::Facade::
        // writer.write_text(&display, vertex_buffer, index_buffer)
    }
}
