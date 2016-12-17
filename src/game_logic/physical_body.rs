use game_logic::actor::Actor;
use engine::generic_object::GenericObject;

#[derive(Clone, Debug)]
pub struct PhysicalBody {
    name: String,
    left_up_corner: [f32; 2],
    right_down_corner: [f32; 2],
    actor: Box<Actor>,
    speed: f32,
}

impl PhysicalBody {
    pub fn new(name: String,
               left_up_corner: [f32; 2],
               right_down_corner: [f32; 2],
               actor: Box<Actor>,
               speed: f32)
               -> PhysicalBody {

        let l_u_c = [left_up_corner[0], left_up_corner[1]];
        let r_d_c = [right_down_corner[0], right_down_corner[1]];

        PhysicalBody {
            name: name,
            left_up_corner: l_u_c,
            right_down_corner: r_d_c,
            actor: actor,
            speed: speed,
        }
    }

    pub fn generate_actor(&self) -> Box<Actor> {

        Box::new(Actor::new((&&self.name).to_string(),
                            [self.actor.get_position().0, self.actor.get_position().1],
                            self.actor.get_texture_id(),
                            [self.actor.get_size().0, self.actor.get_size().1]))
    }

    pub fn get_name(&self) -> &str {
        &&self.name
    }

    pub fn detect_collision(&self, body: &PhysicalBody) -> bool {

        let aa_bb_guest = body.get_aa_bb();

        if self.left_up_corner[0] <= aa_bb_guest.1[0] &&
           self.left_up_corner[1] >= aa_bb_guest.1[1] &&
           self.right_down_corner[0] >= aa_bb_guest.0[0] &&
           self.right_down_corner[1] <= aa_bb_guest.0[1] {
            return true;
        }

        return false;
    }

    pub fn get_aa_bb(&self) -> ([f32; 2], [f32; 2]) {
        (self.left_up_corner, self.right_down_corner)
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::generic_object::GenericObject;
    use game_logic::actor::Actor;

    #[test]
    fn should_generate_actors() {
        let body =
            PhysicalBody::new("toto".to_string(),
                              [0.0, 0.0],
                              [1.0, -1.0],
                              Box::new(Actor::new("toto".to_string(), [0.0, 0.0], 0, [0.1, 0.1])),
                              0.0);
        let actors = body.generate_actor();

        assert_eq!((*actors.as_ref()).get_position(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn should_get_name() {
        let body =
            PhysicalBody::new("toto".to_string(),
                              [0.0, 0.0],
                              [1.0, -1.0],
                              Box::new(Actor::new("toto".to_string(), [0.0, 0.0], 0, [0.1, 0.1])),
                              0.0);
        let name = body.get_name();

        assert_eq!(name, "toto");
    }

    #[test]
    fn should_move_physical_body() {
        let body =
            PhysicalBody::new("toto".to_string(),
                              [0.0, 0.0],
                              [1.0, -1.0],
                              Box::new(Actor::new("toto".to_string(), [0.0, 0.0], 0, [0.1, 0.1])),
                              0.0);
        let actors = body.generate_actor();

        assert!(false);
    }

    #[test]
    fn should_detect_collision() {
        let body1 =
            PhysicalBody::new("toto".to_string(),
                              [0.0, 0.0],
                              [1.0, -1.0],
                              Box::new(Actor::new("toto".to_string(), [0.0, 0.0], 0, [0.1, 0.1])),
                              0.0);
        let body2 =
            PhysicalBody::new("titi".to_string(),
                              [0.0, 0.0],
                              [1.0, -1.0],
                              Box::new(Actor::new("titi".to_string(), [0.0, 0.0], 0, [0.1, 0.1])),
                              0.0);

        let result = body1.detect_collision(&body2);
        assert_eq!(result, true);
    }

    fn should_get_aa_bb() {
        let body =
            PhysicalBody::new("toto".to_string(),
                              [0.0, 0.0],
                              [1.0, -1.0],
                              Box::new(Actor::new("toto".to_string(), [0.0, 0.0], 0, [0.1, 0.1])),
                              0.0);
        let aa_bb = body.get_aa_bb();

        assert_eq!(aa_bb, ([0.0, 0.0], [1.0, -1.0]));
    }
}
