// use movements::State;
use engine::generic_object::GenericObject;
use game_logic::actor::Actor;

pub struct LogicHandler ;

impl LogicHandler {
    // fn go_next()
    pub fn init(&self) -> Vec<Box<GenericObject>> {
        let player = Actor::new("player", [0.0,0.0], "../../content/VFKM2.png", [0.1,0.1]);
        vec![Box::new(player)]
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_return_an_array_of_generic_object(){
        let logic = LogicHandler;
        let array = logic.init();

        // println!("{:?}", array);
        assert_eq!(array.len(),1);
    }
}
