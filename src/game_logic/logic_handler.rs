// use movements::State;
use engine::generic_object::GenericObject;
use game_logic::actor::Actor;
use game_logic::text::Text;
use game_logic::movement::{Move, Movements,State, Fall, Walk};
use game_logic::physical_body::PhysicalBody;
use engine::generic_object_type::GenericObjectType;

// #[derive(Copy)]
pub struct LogicHandler{
    buffer: Vec<PhysicalBody>,
    state_buffer: Movements,
}

impl LogicHandler {
    // fn go_next()
    pub fn new() -> LogicHandler {
        let mut mov = Movements::new();
        LogicHandler{
            buffer: vec![PhysicalBody::new("player".to_string(), [-0.1,0.1], [0.1,-0.1],Box::new(Actor::new("player".to_string(), [0.0,0.0], 3, [0.1,0.1])))],
            // buffer: vec![Box::new(Actor::new("player".to_string(), [0.0,0.0], 3, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.9,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.8,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.7,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.6,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.5,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.4,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.3,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.2,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.1,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [-0.0,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.9,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.8,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.7,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.6,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.5,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.4,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.3,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.2,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.1,0.0], 2, [0.1,0.1])),
            //             Box::new(Actor::new("obstacle".to_string(), [0.0,0.0], 2, [0.1,0.1])),
            //             Box::new(Text::new("fps".to_string(), [-0.5,0.8],"une valeur".to_string()))],
            state_buffer: mov,
        }

    }

    // pub fn get_buffer(&self, time: (f64,f64)) -> &Vec<Box<GenericObject>> {
    //     let mut result : Vec<Box<GenericObject>> = vec![];
    //     for el in &self.buffer {
    //         result.push(el.generate_actor());
    //     }
    //
    //     &result
    // }

    pub fn update(&mut self, time: (f64,f64), keys: &Vec<&str>) -> Vec<Box<GenericObject>>  {

        let mut result : Vec<Box<GenericObject>> = vec![];

        let lists : (Vec<Box<GenericObject>>, Vec<PhysicalBody>) = self.go_threw_buffer(time,keys);

        for el in  &lists.0 {

            match el.get_type() {
                GenericObjectType::Sprite => result.push(Box::new(Actor::new(el.get_name(), [el.get_position().0, el.get_position().1], el.get_texture_id(),[0.1,0.1]))),
                GenericObjectType::Text => result.push(Box::new(Text::new(el.get_name(), [el.get_position().0, el.get_position().1], el.get_description())))
            }
        }

        self.buffer.clear();

        for el in lists.1 {
            self.buffer.push(el);
        }

        result

    }

    fn go_threw_buffer(&mut self, time: (f64,f64), keys: &Vec<&str>) -> (Vec<Box<GenericObject>>, Vec<PhysicalBody>) {

        let mut lst_physical_bodies : Vec<PhysicalBody>  = vec![];
        let mut result : Vec<Box<GenericObject>> = vec![];
        let mut name : &str;

        for e in &mut self.buffer {
            let el : Box<Actor> = e.generate_actor();
            name = e.get_name();

            if name == "obstacle" {
                let mut pos = el.get_position().0 - 0.1 * time.1 as f32;
                if pos <= -1.5 {
                    pos = 1.5;
                }
                let new_position = [pos,  el.get_position().1];
                lst_physical_bodies.push(PhysicalBody::new(name.to_string(), [-0.1,0.1],[0.1,-0.1] ,
                    Box::new(Actor::new(name.to_string(), new_position, 2,[0.1,0.1]))));

            } else if name == "player"{
                if self.state_buffer.get_status() == Move::Fall {
                    if el.get_position().1 <= -0.8 {
                        self.state_buffer.update_status(Move::Walk);
                    }
                    lst_physical_bodies.push(PhysicalBody::new(name.to_string(), [-0.1,0.1],[0.1,-0.1] ,
                    Box::new(Actor::new(name.to_string(), [-0.8,el.get_position().1 - 0.15 * time.1 as f32], 3,[0.2,0.2]))));

                } else if self.state_buffer.get_status() == Move::Walk {
                    lst_physical_bodies.push(PhysicalBody::new(name.to_string(), [-0.1,0.1],[0.1,-0.1] ,
                    Box::new(Actor::new(name.to_string(), [-0.8,-0.8], 3,[0.2,0.2]))));

                }else {
                    lst_physical_bodies.push(PhysicalBody::new(name.to_string(), [-0.1,0.1],[0.1,-0.1] ,
                    Box::new(Actor::new(name.to_string(), [-0.8,0.0], 3,[0.2,0.2]))));
                }

            }


            for el in &lst_physical_bodies {
                result.push(el.generate_actor());
            }

            result.push(Box::new(Text::new("fps".to_string(), [-0.5,0.8],format!("fps : `{fps:.*}`",2,fps=time.0))));
        }

        (result, lst_physical_bodies)
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

        logic.update((0.0,0.0), &vec![]);
        assert_eq!(logic.get_buffer((0.0,0.0)).len(),1);
    }
}
