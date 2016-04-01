// use movements::State;
use engine::generic_object::GenericObject;
use game_logic::actor::Actor;

pub struct LogicHandler ;

impl LogicHandler {
    // fn go_next()
    fn init(&self) -> Vec<Box<GenericObject>> {
        let player = Actor::new("player", [0.0,0.0], "../../content/VFKM2.png", [0.1,0.1]);
    }
}
