// use movements::State;
use engine::generic_object::GenericObject;
use game_logic::actor::Actor;
use game_logic::text::Text;
use game_logic::movement::{Move, Movements,State, Fall, Walk};


// #[derive(Copy)]
pub struct LogicHandler{
    buffer: Vec<Box<GenericObject>>,
    state_buffer: Movements,
}

impl LogicHandler {
    // fn go_next()
    pub fn new() -> LogicHandler {
        let mut mov = Movements::new();
        LogicHandler{
            buffer: vec![Box::new(Actor::new("player".to_string(), [0.0,0.0], 3, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.9,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.8,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.7,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.6,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.5,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.4,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.3,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.2,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.1,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [-0.0,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.9,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.8,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.7,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.6,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.5,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.4,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.3,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.2,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.1,0.0], 2, [0.1,0.1])),
                        Box::new(Actor::new("obstacle".to_string(), [0.0,0.0], 2, [0.1,0.1])),
                        Box::new(Text::new("fps".to_string(), [-0.5,0.8],"une valeur".to_string()))],
            state_buffer: mov,
        }

    }

    pub fn get_buffer(&self, time: (f64,f64)) -> &Vec<Box<GenericObject>> {

        &self.buffer
    }

    pub fn update(&mut self, time: (f64,f64), keys: &Vec<&str>) -> &Vec<Box<GenericObject>>  {

        let result = self.go_threw_buffer(time, keys);

        self.buffer.clear();
        self.buffer = result;

        &self.buffer

    }

    fn go_threw_buffer(&mut self, time: (f64,f64), keys: &Vec<&str>) -> Vec<Box<GenericObject>> {

        let mut result : Vec<Box<GenericObject>>  = vec![];
        let mut name : String;
        for el in &self.buffer {
            name = el.get_name();

            if name == "obstacle" {
                let mut pos = el.get_position().0 - 0.1 * time.1 as f32;
                if pos <= -1.5 {
                    pos = 1.5;
                }
                let new_position = [pos,  el.get_position().1];
                result.push(Box::new(Actor::new(name, new_position, 2,[0.1,0.1])));

            } else if name == "player"{
                if self.state_buffer.get_status() == Move::Fall {
                    if el.get_position().1 <= -0.8 {
                        self.state_buffer.update_status(Move::Walk);
                    }
                    result.push(Box::new(Actor::new(name, [-0.8,el.get_position().1 - 0.15 * time.1 as f32], 3,[0.2,0.2])));


                } else if self.state_buffer.get_status() == Move::Walk {
                    result.push(Box::new(Actor::new(name, [-0.8,-0.8], 3,[0.2,0.2])));

                }else {
                    result.push(Box::new(Actor::new(name, [-0.8,0.0], 3,[0.2,0.2])));

                }

            } else {
                result.push(Box::new(Text::new("fps".to_string(), [-0.5,0.8],format!("fps : `{fps:.*}`",2,fps=time.0))));
            }

        }

        return result;
    }


}

#[cfg(test)]
mod tests{
    use super::*;

    #[ignore]
    #[test]
    fn should_return_an_array_of_generic_object(){
        let logic = LogicHandler::new();
        // let array = logic.init();

        // assert_eq!(array.len(),1);
    }

    #[test]
    fn should_update_something(){
        let mut logic = LogicHandler::new();

        logic.update((0.0,0.0));
        assert_eq!(logic.get_buffer((0.0,0.0)).len(),1);
    }
}
