// use movements::State;
use engine::generic_object::GenericObject;
use game_logic::actor::Actor;

// #[derive(Copy)]
pub struct LogicHandler{
    buffer: Vec<Box<GenericObject>>,
}

impl LogicHandler {
    // fn go_next()
    pub fn new() -> LogicHandler {
        LogicHandler{
            buffer: vec![Box::new(Actor::new("player".to_string(), [0.0,0.0], "../../content/hero.png", [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.9,0.0], "../../content/NatureForests.png", [0.1,0.1])),],
        }

    }

    pub fn get_buffer(&self) -> Vec<Box<GenericObject>> {
        let mut result : Vec<Box<GenericObject>>  = vec![];
        let mut name : String;
        for el in &self.buffer {
            name = el.get_name();
            result.push(Box::new(Actor::new(name, [el.get_position().0,el.get_position().1], "../../content/VFKM2.png",[0.1,0.1])));
        }

        result
    }

    pub fn update(&mut self, delta: f64)  {
        println!("position : {:?}", self.buffer[0].get_position());

        let mut result : Vec<Box<GenericObject>>  = vec![];
        let mut name : String;

        for el in &self.buffer {
            name = el.get_name();

            if name == "obstacle" {

                let new_position = [el.get_position().0 - 1.0 * delta as f32,  el.get_position().1];
                result.push(Box::new(Actor::new(name, new_position, "../../content/NatureForests.png",[0.1,0.1])));

            } else {
                result.push(Box::new(Actor::new(name, [0.0,0.0], "../../content/hero.png",[0.1,0.1])));

            }
        }
        self.buffer.clear();
        self.buffer = result;

    }


}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_return_an_array_of_generic_object(){
        let logic = LogicHandler::new();
        let array = logic.init();
        // println!("{:?}", array);

        assert_eq!(array.len(),1);
    }

    #[test]
    fn should_update_something(){
        let logic = LogicHandler::new();

        logic.update();
        assert_eq(logic.get_buffer().len(),1);
    }
}
