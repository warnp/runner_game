use vertex::Vertex;
use sprite::Sprite;

pub struct TextWriter<'a> {
    pub string_name: &'a str,
    pub image_index: u16,
    pub image_size: (u16, u16),
    pub character_size: (u16, u16),
    pub text_size: f32,
    pub text_origin: (f32, f32),
}

impl<'a> TextWriter<'a> {
    pub fn new(image_index: u16,
               image_size: (u16, u16),
               character_size: (u16, u16),
               text_size: f32,
               text_origin: (f32, f32),
               string_name: &'a str)
               -> TextWriter<'a> {
        TextWriter {
            image_index: image_index,
            image_size: image_size,
            character_size: character_size,
            text_size: text_size,
            text_origin: text_origin,
            string_name: string_name,
        }
    }

    pub fn get_string(&self, entry: &str) -> Vec<Sprite> {
        let mut return_vec = Vec::new();

        for s in entry.chars() {

            match s {
                '0' => {
                    return_vec.push(((0.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (0.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '1' => {
                    return_vec.push(((1.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '2' => {
                    return_vec.push(((2.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '3' => {
                    return_vec.push(((3.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '4' => {
                    return_vec.push(((4.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '5' => {
                    return_vec.push(((5.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '6' => {
                    return_vec.push(((6.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '7' => {
                    return_vec.push(((7.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '8' => {
                    return_vec.push(((8.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                '9' => {
                    return_vec.push(((9.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      13.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32)))
                }
                'a' => {
                    return_vec.push(((1.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'b' => {
                    return_vec.push(((2.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'c' => {
                    return_vec.push(((3.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'd' => {
                    return_vec.push(((4.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'e' => {
                    return_vec.push(((5.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'f' => {
                    return_vec.push(((6.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'g' => {
                    return_vec.push(((7.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'h' => {
                    return_vec.push(((8.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'i' => {
                    return_vec.push(((9.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'j' => {
                    return_vec.push(((10.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'k' => {
                    return_vec.push(((11.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (12.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (12.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'l' => {
                    return_vec.push(((12.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (13.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (13.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (12.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'm' => {
                    return_vec.push(((13.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (14.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (14.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (13.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'n' => {
                    return_vec.push(((14.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (15.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (15.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (14.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'o' => {
                    return_vec.push(((15.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (16.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (16.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (15.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32)))
                }
                'p' => {
                    return_vec.push(((0.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (0.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'q' => {
                    return_vec.push(((1.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'r' => {
                    return_vec.push(((2.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                's' => {
                    return_vec.push(((3.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                't' => {
                    return_vec.push(((4.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'u' => {
                    return_vec.push(((5.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'v' => {
                    return_vec.push(((6.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'w' => {
                    return_vec.push(((7.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'x' => {
                    return_vec.push(((8.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'y' => {
                    return_vec.push(((9.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'z' => {
                    return_vec.push(((10.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      9.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      8.0 / self.character_size.0 as f32)))
                }
                'A' => {
                    return_vec.push(((1.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'B' => {
                    return_vec.push(((2.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'C' => {
                    return_vec.push(((3.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'D' => {
                    return_vec.push(((4.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'E' => {
                    return_vec.push(((5.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'F' => {
                    return_vec.push(((6.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'G' => {
                    return_vec.push(((7.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'H' => {
                    return_vec.push(((8.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'I' => {
                    return_vec.push(((9.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'J' => {
                    return_vec.push(((10.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'K' => {
                    return_vec.push(((11.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (12.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (12.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'L' => {
                    return_vec.push(((12.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (13.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (13.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (12.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'M' => {
                    return_vec.push(((13.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (14.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (14.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (13.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'N' => {
                    return_vec.push(((14.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (15.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (15.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (14.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'O' => {
                    return_vec.push(((15.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (16.0 / self.character_size.0 as f32,
                                      12.0 / self.character_size.0 as f32),
                                     (16.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (15.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32)))
                }
                'P' => {
                    return_vec.push(((0.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (0.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'Q' => {
                    return_vec.push(((1.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (1.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'R' => {
                    return_vec.push(((2.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'S' => {
                    return_vec.push(((3.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (4.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (2.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'T' => {
                    return_vec.push(((4.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (3.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'U' => {
                    return_vec.push(((5.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (5.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'V' => {
                    return_vec.push(((6.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (6.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'W' => {
                    return_vec.push(((7.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (7.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'X' => {
                    return_vec.push(((8.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (8.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'Y' => {
                    return_vec.push(((9.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (9.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                'Z' => {
                    return_vec.push(((10.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      11.0 / self.character_size.0 as f32),
                                     (11.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32),
                                     (10.0 / self.character_size.0 as f32,
                                      10.0 / self.character_size.0 as f32)))
                }
                _ => {
                    return_vec.push(((0.0 / self.character_size.0 as f32,
                                      0.0 / self.character_size.0 as f32),
                                     (0.0 / self.character_size.0 as f32,
                                      0.0 / self.character_size.0 as f32),
                                     (0.0 / self.character_size.0 as f32,
                                      0.0 / self.character_size.0 as f32),
                                     (0.0 / self.character_size.0 as f32,
                                      0.0 / self.character_size.0 as f32)))
                }

            }

        }
        let mut sprites_vec = Vec::new();
        let mut i = 0.0;
        for  st in &return_vec {
            sprites_vec.push(Sprite::new("toto",
                                         self.text_origin.0 + self.text_size * i + 1.0,
                                         self.text_origin.1,
                                         [1.0, 1.0, 1.0, 1.0],
                                         0,
                                         (self.text_size, self.text_size)));
                                         i = i +1.0 ;
        }

        sprites_vec
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    extern crate glium;
    // This test assume your bmpfont map is 256*256 with char 16*16
    #[test]
    fn should_set_charmap() {
        let writer = TextWriter::new(0, (256, 256), (16, 16), 1.0, (0.0,0.0), "toto");

        assert!(writer.image_index == 0);
        assert!(writer.image_size == (256, 256));
        assert!(writer.character_size == (16, 16));

    }

    #[test]
    fn should_give_characters_coordinate() {
        let writer = TextWriter::new(0, (256, 256), (16, 16), 1.0, (0.0,0.0), "toto");

        let coordinates = writer.get_string("Blop");

        assert_eq!(coordinates.len(), 4);
        // TODO A modifier les coordonées !!!
        // println!("{:?}", coordinates[0]);
        // assert!(coordinates[0] ==
        //         ((2.0 / 16.0, 12.0 / 16.0),
        //          (3.0 / 16.0, 12.0 / 16.0),
        //          (3.0 / 16.0, 11.0 / 16.0),
        //          (2.0 / 16.0, 11.0 / 16.0)));
        // assert!(coordinates[1] ==
        //         ((12.0 / 16.0, 10.0 / 16.0),
        //          (13.0 / 16.0, 10.0 / 16.0),
        //          (13.0 / 16.0, 9.0 / 16.0),
        //          (12.0 / 16.0, 9.0 / 16.0)));
        // assert!(coordinates[2] ==
        //         ((15.0 / 16.0, 10.0 / 16.0),
        //          (16.0 / 16.0, 10.0 / 16.0),
        //          (16.0 / 16.0, 9.0 / 16.0),
        //          (15.0 / 16.0, 9.0 / 16.0)));
        // assert!(coordinates[3] ==
        //         ((0.0 / 16.0, 9.0 / 16.0),
        //          (1.0 / 16.0, 9.0 / 16.0),
        //          (1.0 / 16.0, 8.0 / 16.0),
        //          (0.0 / 16.0, 8.0 / 16.0)));

    }
}
