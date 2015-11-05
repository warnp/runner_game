pub struct CollisionMesh {
    aa: [f32; 2],
    bb: [f32; 2],
}

impl CollisionMesh {


    fn detect_collide(&self,aa: [f32; 2], bb: [f32; 2]) -> bool {
        if self.aa[0] <= bb[0] &&
            self.aa[1] >= bb[1] &&
            self.bb[0] >= aa[0] &&
            self.bb[1] <= aa[1] {
                return true;
            }
        return false;
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_collide(){
        let collider = CollisionMesh {
            aa: [-0.5,0.1],
            bb: [1.0,-1.0],
        };

        assert!(collider.detect_collide([-1.0, 1.0], [0.0,0.0]));
    }

    #[test]
    #[should_panic]
    fn should_not_collide(){
        let collider = CollisionMesh {
            aa: [-0.5,0.1],
            bb: [1.0,-1.0],
        };

        assert_eq!(collider.detect_collide([-1.0, 1.0], [0.0,0.0]), false);
    }
}
