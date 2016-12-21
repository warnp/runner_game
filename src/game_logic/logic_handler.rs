extern crate rand;

use rand::Rng;
use engine::generic_object::GenericObject;
use game_logic::actor::Actor;
use game_logic::text::Text;
use game_logic::movement::{Move, Movements, State, Fall, Walk};
use game_logic::physical_body::PhysicalBody;
use engine::generic_object_type::GenericObjectType;

// #[derive(Copy)]
pub struct LogicHandler {
    buffer: Vec<PhysicalBody>,
    state_buffer: Movements,
    debug: bool,
    gravity: f32,
}

impl LogicHandler {
    // fn go_next()
    pub fn new() -> LogicHandler {
        let mut mov = Movements::new();
        let mut x = rand::thread_rng();


        let mut buffer = vec![PhysicalBody::new("player".to_string(),
                                                [-0.025, 0.05],
                                                [0.025, -0.05],
                                                Box::new(Actor::new("player".to_string(),
                                                                    [0.0, 0.0],
                                                                    3,
                                                                    [0.1, 0.1])),
                                                0.0)];

        //  for i in 0..10 {
        //      let height = x.gen::<u8>();
        //      let size_x = x.gen::<u8>();
        //      let size_y = x.gen::<u8>();
        //
        //      let size_x = (size_x as f32) / 511.0;
        //      let size_y = (size_y as f32) / 383.0;
        //      let height = (height as f32 - 128.0) / 127.0;
        //      buffer.push(PhysicalBody::new("obstacle".to_string(),
        //      [1.0-size_x/2.0,height + size_y /2.0],
        //       [1.0+size_x/2.0,height-size_y/2.0],
        //       Box::new(Actor::new("obstacle".to_string(),
        //                 [1.0,height],
        //                  2,
        //                  [size_x,size_y]))));
        //  }
        buffer.push(PhysicalBody::new("obstacle".to_string(),
                                      [-0.1, 0.1],
                                      [0.1, -0.1],
                                      Box::new(Actor::new("obstacle".to_string(),
                                                          [1.0, -0.8],
                                                          2,
                                                          [0.2, 0.2])),
                                      0.0));
        buffer.push(PhysicalBody::new("obstacle".to_string(),
                                      [-0.7, 0.1],
                                      [0.7, -0.1],
                                      Box::new(Actor::new("obstacle1".to_string(),
                                                          [-0.5, -0.8],
                                                          2,
                                                          [1.4, 0.2])),
                                      0.0));

        LogicHandler {
            buffer: buffer,
            state_buffer: mov,
            debug: false,
            gravity: 0.02,
        }

    }

    pub fn update(&mut self, time: (f64, f64), keys: &str) -> Vec<Box<GenericObject>> {

        let mut result: Vec<Box<GenericObject>> = vec![];

        let lists: (Vec<Box<GenericObject>>, Vec<PhysicalBody>) = self.go_threw_buffer(time, keys);
        if LogicHandler::detect_collision_with_player(&lists.1, self) {
            self.debug = true;

        }

        for el in &lists.0 {
            match el.get_type() {
                GenericObjectType::Sprite => {
                    result.push(Box::new(Actor::new(el.get_name(),
                                                    [el.get_position().0, el.get_position().1],
                                                    el.get_texture_id(),
                                                    [el.get_size().0, el.get_size().1])))
                }
                GenericObjectType::Text => {
                    result.push(Box::new(Text::new(el.get_name(),
                                                   [el.get_position().0, el.get_position().1],
                                                   el.get_description())))
                }
            }
        }

        self.buffer.clear();

        for el in lists.1 {
            self.buffer.push(el);
        }

        result
    }

    pub fn get_debug(&self) -> bool {
        self.debug
    }

    fn go_threw_buffer(&mut self,
                       time: (f64, f64),
                       keys: &str)
                       -> (Vec<Box<GenericObject>>, Vec<PhysicalBody>) {

        let mut lst_physical_bodies: Vec<PhysicalBody> = vec![];
        let mut result: Vec<Box<GenericObject>> = vec![];
        let mut name: &str;

        // println!("{:#?}", keys);

        for e in &mut self.buffer {
            let el: Box<Actor> = e.generate_actor();
            name = e.get_name();
            if name == "obstacle" {
                let mut pos = el.get_position().0 - 0.1 * time.1 as f32;
                if pos <= -1.5 {
                    pos = 1.5;
                }
                let new_position = [pos, el.get_position().1];

                lst_physical_bodies.push(PhysicalBody::new(name.to_string(),
                                            [new_position[0] - el.get_size().0 / 2.0,
                                             new_position[1] + el.get_size().1 / 2.0],
                                            [new_position[0] + el.get_size().0 / 2.0,
                                             new_position[1] - el.get_size().1 / 2.0],
                                            Box::new(Actor::new(name.to_string(),
                                                                new_position,
                                                                2,
                                                                [el.get_size().0,
                                                                 el.get_size().1])),
                                            0.0));

            } else if name == "player" {
                if self.state_buffer.get_status() == Move::Fall {
                    if el.get_position().1 <= -0.8 {
                        self.state_buffer.update_status(Move::Walk);
                    }
                    let mut speed = 0.0;
                    if time.1 > 0.0 {
                        speed = e.get_speed() + self.gravity;
                    }
                    lst_physical_bodies.push(PhysicalBody::new(name.to_string(),
                                                [el.get_position().0 - 0.025,
                                                 el.get_position().1 + 0.05],
                                                [el.get_position().0 + 0.025,
                                                 el.get_position().1 - 0.05],
                                                Box::new(Actor::new(name.to_string(),
                                                                    [-0.8,
                                                                     el.get_position().1 -
                                                                     speed * time.1 as f32],
                                                                    3,
                                                                    [el.get_size().0,
                                                                     el.get_size().1])),
                                                speed));

                } else if self.state_buffer.get_status() == Move::Walk {
                    if keys == "space_press" {
                        self.state_buffer.update_status(Move::Jump);
                        lst_physical_bodies.push(PhysicalBody::new(name.to_string(),
                                                    [el.get_position().0 - 0.025,
                                                     el.get_position().1 + 0.05],
                                                    [el.get_position().0 + 0.025,
                                                     el.get_position().1 - 0.05],
                                                    Box::new(Actor::new(name.to_string(),
                                                                        [-0.8,
                                                                         el.get_position().1],
                                                                        3,
                                                                        [el.get_size().0,
                                                                         el.get_size().1])),
                                                    1.0));
                    } else {
                        lst_physical_bodies.push(PhysicalBody::new(name.to_string(),
                                                    [el.get_position().0 - 0.025,
                                                     el.get_position().1 + 0.05],
                                                    [el.get_position().0 + 0.025,
                                                     el.get_position().1 - 0.05],
                                                    Box::new(Actor::new(name.to_string(),
                                                                        [-0.8,
                                                                         el.get_position().1],
                                                                        3,
                                                                        [el.get_size().0,
                                                                         el.get_size().1])),
                                                    0.0));
                    }

                } else if self.state_buffer.get_status() == Move::Jump {
                    let speed = e.get_speed() - self.gravity;
                    lst_physical_bodies.push(PhysicalBody::new(name.to_string(),
                                                [el.get_position().0 - 0.025,
                                                 el.get_position().1 + 0.05],
                                                [el.get_position().0 + 0.025,
                                                 el.get_position().1 - 0.05],
                                                Box::new(Actor::new(name.to_string(),
                                                                    [-0.8,
                                                                     speed * time.1 as f32 +
                                                                     el.get_position().1],
                                                                    3,
                                                                    [el.get_size().0,
                                                                     el.get_size().1])),
                                                speed));
                    if e.get_speed() <= 0.0 {
                        self.state_buffer.update_status(Move::Fall);
                    }
                }

            }

            for el in &lst_physical_bodies {
                result.push(el.generate_actor());
            }

            result.push(Box::new(Text::new("fps".to_string(),
                                           [-0.5, 0.8],
                                           format!("fps : `{fps:.*}`", 2, fps = time.0))));
        }

        (result, lst_physical_bodies)
    }

    fn detect_collision_with_player(physical_buffer: &Vec<PhysicalBody>,
                                    logic_handler: &mut LogicHandler)
                                    -> bool {
        let player = physical_buffer.iter().find(|x| x.get_name() == "player");
        match (player) {
            Some(p) => {
                for o in physical_buffer {
                    if o.get_name() != "player" &&
                       p.get_collision_ray([0.0, -0.05], [0.0, -0.05], &o) &&
                       logic_handler.state_buffer.get_status() == Move::Fall {
                        logic_handler.state_buffer.update_status(Move::Walk);
                    }

                    if o.get_name() != "player" &&
                       !p.get_collision_ray([0.0, -0.05], [0.0, -0.05], &o) &&
                       logic_handler.state_buffer.get_status() == Move::Walk &&
                       logic_handler.state_buffer.get_status() != Move::Jump {
                        logic_handler.state_buffer.update_status(Move::Fall);

                    }

                    if o.get_name() != "player" && p.detect_collision(o) {
                        println!("Collision!!!");
                        return true;
                    }
                }
            }
            None => (),
        }
        // println!("{:?}", Some(player).get_name());
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn should_return_an_array_of_generic_object() {
        let logic = LogicHandler::new();
        // let array = logic.init();

        // assert_eq!(array.len(),1);
    }

    #[ignore]
    #[test]
    fn should_update_something() {
        let mut logic = LogicHandler::new();

        // logic.update((0.0, 0.0), &vec![]);
        // assert_eq!(logic.get_buffer((0.0,0.0)).len(),1);
    }
}
