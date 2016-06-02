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
            buffer: vec![Box::new(Actor::new("player".to_string(), [0.0,0.0], 3, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.9,0.0], 0, [0.1,0.1])),],
        }

    }

    pub fn get_buffer(&self) -> Vec<Box<GenericObject>> {
        // let mut result : Vec<Box<GenericObject>>  = vec![];

        let result = self.go_threw_buffer(0.0);

        result
    }

    pub fn update(&mut self, delta: f64)  {
        // println!("position : {:?}", self.buffer[0].get_position());

        // let mut result : Vec<Box<GenericObject>>  = vec![];

        let result = self.go_threw_buffer(delta);

        self.buffer.clear();
        self.buffer = result;

    }

    fn go_threw_buffer(&self, delta: f64) -> Vec<Box<GenericObject>> {

        let mut result : Vec<Box<GenericObject>>  = vec![];
        let mut name : String;

        for el in &self.buffer {
            name = el.get_name();

            if name == "obstacle" {

                let new_position = [el.get_position().0 - 1.0 * delta as f32,  el.get_position().1];
                result.push(Box::new(Actor::new(name, new_position, 0,[0.1,0.1])));

            } else {
                result.push(Box::new(Actor::new(name, [0.0,0.0], 3,[0.2,0.2])));

            }

        }

        return result;
    }


}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_return_an_array_of_generic_object(){
        let logic = LogicHandler::new();
        let array = logic.init();

        assert_eq!(array.len(),1);
    }

    #[test]
    fn should_update_something(){
        let logic = LogicHandler::new();

        logic.update();
        assert_eq(logic.get_buffer().len(),1);
    }
}
