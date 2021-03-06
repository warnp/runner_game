use std::cmp::{Ord, Ordering};
use engine::vertex::Vertex;
use engine::graphic_item::GraphicItem;
use engine::collision::CollisionMesh;
use engine::layer_type::LayerType;

extern crate glium;


#[derive( Clone, Debug)]
pub struct Sprite {
    pub vertices: [Vertex; 4],
    pub indices: [u16; 6],
    pub name: String, /* pub transform: [[f32; 4]; 4],
                        * pub display: &glium::glutin::WindowBuilder, */
    pub order: u8,
}

impl Sprite {
    pub fn new(name: String,
               x: f32,
               y: f32,
               color: [f32; 4],
               tex_id: u32,
               size: (f32, f32),
               tex_coord:((f32,f32,),(f32,f32),(f32,f32),(f32,f32)),
               order: u8)
               -> Sprite {
        Sprite {
            vertices: [Vertex {
                           position: [-0.5 * size.0 + x, 0.5 * size.1 + y,0.0],
                           normal: [0.0, 0.0, -1.0],
                           color: color,
                           tex_coords: [tex_coord.0 .0, tex_coord.0 .1],
                           i_tex_id: tex_id,
                       },
                       Vertex {
                           position: [0.5 * size.0 + x, 0.5 * size.1 + y,0.0],
                           normal: [0.0, 0.0, -1.0],
                           color: color,
                           tex_coords: [tex_coord.1 .0, tex_coord.1 .1],
                           i_tex_id: tex_id,
                       },
                       Vertex {
                           position: [0.5 * size.0 + x, -0.5 * size.1 + y,0.0],
                           normal: [0.0, 0.0, -1.0],
                           color: color,
                           tex_coords: [tex_coord.2 .0, tex_coord.2 .1],
                           i_tex_id: tex_id,
                       },
                       Vertex {
                           position: [-0.5 * size.0 + x, -0.5 * size.1 + y,0.0],
                           normal: [0.0, 0.0, -1.0],
                           color: color,
                           tex_coords: [tex_coord.3 .0, tex_coord.3 .1],
                           i_tex_id: tex_id,
                       }],
            indices: [0, 1, 2, 0, 2, 3],
            name: name, // transform: transform,
            order: order,
        }

    }
}


impl GraphicItem for Sprite {
    fn get_position(&self) -> [f32; 2] {

        let x = (self.vertices[0].position[0] + self.vertices[1].position[0] +
                 self.vertices[2].position[0] +
                 self.vertices[3].position[0]) as f32 / 4.0;
        let y = (self.vertices[0].position[1] + self.vertices[1].position[1] +
                 self.vertices[2].position[1] +
                 self.vertices[3].position[1]) as f32;
        [x, y]
    }

    fn get_layer(&self) -> LayerType{
        LayerType::UI
    }
}

impl CollisionMesh for Sprite {
    fn detect_collide(&self, aa: [f32; 2], bb: [f32; 2]) -> bool {
        if self.vertices[0].position[0] <= bb[0] && self.vertices[0].position[1] >= bb[1] &&
           self.vertices[2].position[0] >= aa[0] &&
           self.vertices[2].position[1] <= aa[1] {
            return true;
        }
        return false;
    }

    fn get_aa_bb(&self) -> ([f32; 3], [f32; 3]) {
        let aa = self.vertices[0].position;
        let bb = self.vertices[2].position;

        (aa, bb)
    }
}

impl Ord for Sprite {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.order, &self.name).cmp(&(other.order, &other.name))
    }
}

impl PartialOrd for Sprite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sprite {
    fn eq(&self, other: &Self) -> bool {
        (self.order, &self.name) == (other.order, &other.name)
    }
}

impl Eq for Sprite {}


#[cfg(test)]
mod tests {
    use super::*;

    use engine::collision::CollisionMesh;
    use engine::graphic_item::GraphicItem;
    use std::cmp::Ordering;

    #[test]
    fn should_get_ordering() {
        let a = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (1.0, 1.0),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 0);
        let b = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (1.0, 1.0),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 1);

        let ordered = a.cmp(&b);
        assert!(ordered == Ordering::Less);
    }

    #[test]
    fn should_get_equality() {
        let a = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (1.0, 1.0),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 0);
        let b = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (1.0, 1.0),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 0);

        let ordered = a.eq(&b);
        assert!(ordered == true);
    }

    #[test]
    fn should_calculate_center_of_sprite_position() {
        // Given
        let s = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (1.0, 1.0),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 0);
        // when
        let position_result = s.get_position();

        // then
        assert_eq!(position_result, [0.0, 0.0]);
    }

    #[test]
    fn should_collide() {
        let s = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (1.0, 1.0),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 0);

        assert!(s.detect_collide([-0.10, 1.0], [0.0, 0.0]));
    }

    #[test]
    #[should_panic]
    fn should_not_collide() {
        let s = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (0.1, 0.1),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 0);

        assert!(s.detect_collide([1.0, -1.0], [2.0, -3.0]));
    }

    #[test]
    fn should_get_aa_bb_positions() {
        let s = Sprite::new("toto".to_string(), 0.0, 0.0, [1.0, 0.0, 0.0, 1.0], 0, (1.0, 1.0),((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)), 0);

        let aabb = s.get_aa_bb();

        assert!(aabb.0 == [-0.5, 0.5]);
        assert!(aabb.1 == [0.5, -0.5]);
    }


}
