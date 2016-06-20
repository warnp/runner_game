use  game_logic::actor::Actor;
use  engine::generic_object::GenericObject;

#[derive(Clone, Debug)]
pub struct PhysicalBody {
    name: String,
    left_up_corner: [f32;2],
    right_down_corner: [f32;2],
    actor : Box<Actor>,

}

impl PhysicalBody {
    pub fn new(name: String, left_up_corner: [f32;2], right_down_corner: [f32; 2], actor: Box<Actor>) -> PhysicalBody {

        let l_u_c = [actor.get_position().0 + left_up_corner[0], actor.get_position().1 + left_up_corner[1]];
        let r_d_c = [actor.get_position().0 + right_down_corner[0], actor.get_position().1 + right_down_corner[1]];

        PhysicalBody {
            name: name,
            left_up_corner: l_u_c,
            right_down_corner: r_d_c,
            actor: actor,
        }
    }

    pub fn generate_actor(&self) -> Box<Actor> {

        Box::new(Actor::new((&&self.name).to_string(),
         [self.actor.get_position().0,self.actor.get_position().1],
         self.actor.get_texture_id(),
         [0.1,0.1]))
    }

    pub fn get_name(&self) -> &str {
        &&self.name
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_generate_actors(){
        let body = PhysicalBody::new("toto".to_string(), [0.0,0.0],[1.0,-1.0], Box::new(Actor::new("toto", [0.0,0.0], "toto.jpg", [0.1,0.1])));
        let actors = body.generate_actor();

        assert_eq!(actors.get_position(), (0.5,-0.5,0.0));
    }

    #[test]
    fn should_get_name(){
        let body = PhysicalBody::new("toto".to_string(), [0.0,0.0],[1.0,-1.0], Box::new(Actor::new("toto", [0.0,0.0], "toto.jpg", [0.1,0.1])));
        let name = body.get_name();

        assert_eq!(name, "toto");
    }

    #[test]
    fn should_move_physical_body() {
        let body = PhysicalBody::new("toto".to_string(), [0.0,0.0],[1.0,-1.0], Box::new(Actor::new("toto", [0.0,0.0], "toto.jpg", [0.1,0.1])));
        let actors = body.generate_actor();

        assert!(false);
    }

    #[test]
    fn should_detect_collision() {

    }
}
