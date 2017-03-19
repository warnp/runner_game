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
    pub fn new() -> LogicHandler {
        let mut mov = Movements::new();
        let mut x = rand::thread_rng();


        let mut buffer = vec![PhysicalBody::new("player".to_string(),
                                                [-0.025, 0.05],
                                                [0.025, -0.05],
                                                Box::new(Actor::new("player".to_string(),
                                                                    [0.0, 0.0],
                                                                    3,
                                                                    [0.1, 0.1],
                                                                    ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)))),
                                                0.0)];

        buffer.push(PhysicalBody::new("obstacle".to_string(),
                                      [-0.1, 0.1],
                                      [0.1, -0.1],
                                      Box::new(Actor::new("obstacle".to_string(),
                                                          [1.0, -0.8],
                                                          2,
                                                          [0.2, 0.2],
                                                          ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)))),
                                      0.0));
        buffer.push(PhysicalBody::new("obstacle".to_string(),
                                      [-0.7, 0.1],
                                      [0.7, -0.1],
                                      Box::new(Actor::new("obstacle1".to_string(),
                                                          [-0.5, -0.8],
                                                          2,
                                                          [1.4, 0.2],
                                                          ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)))),
                                      0.0));
        buffer.push(PhysicalBody::new("obstacle".to_string(),
                                      [-0.2, 0.3],
                                      [0.2, -0.3],
                                      Box::new(Actor::new("obstacle1".to_string(),
                                                          [0.1, -0.8],
                                                          2,
                                                          [0.4, 0.6],
                                                          ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)))),
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

        for el in &lists.0 {
            match el.get_type() {
                GenericObjectType::Sprite => {
                    result.push(Box::new(Actor::new(el.get_name(),
                                                    [el.get_position().0, el.get_position().1],
                                                    el.get_texture_id(),
                                                    [el.get_size().0, el.get_size().1],
                                                    ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)))))
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

        // Inserer ici un générateur de blocs toute les x secondes grâce au timer?
        for e in self.buffer.iter().filter(|x| x.get_name() == "obstacle") {
            let el: Box<Actor> = e.generate_actor();
            let mut pos = el.get_position().0 - 0.1 * time.1 as f32;
            if pos <= -1.5 {
                pos = 1.5;
            }
            let new_position = [pos, el.get_position().1];

            lst_physical_bodies.push(PhysicalBody::new(e.get_name().to_string(),
                                                       [new_position[0] - el.get_size().0 / 2.0,
                                                           new_position[1] + el.get_size().1 / 2.0],
                                                       [new_position[0] + el.get_size().0 / 2.0,
                                                           new_position[1] - el.get_size().1 / 2.0],
                                                       Box::new(Actor::new(e.get_name()
                                                                               .to_string(),
                                                                           new_position,
                                                                           2,
                                                                           [el.get_size().0,
                                                                               el.get_size().1],
                                                                           ((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)))),
                                                       0.0));
        }

        let player = self.buffer.iter().find(|x| x.get_name() == "player").unwrap();

        let mut collide_one_floor: bool = false;
        for o in &lst_physical_bodies {
            if player.get_collision_ray([0.0, -0.015], [0.0, -0.015], o) {
                self.state_buffer.update_status(Move::Walk);
                collide_one_floor = true;
            }
        }

        if !collide_one_floor && self.state_buffer.get_status() != Move::Jump &&
            self.state_buffer.get_status() != Move::Fall {
            self.state_buffer.update_status(Move::Fall)
        }

        let el: Box<Actor> = player.generate_actor();
        //---------------------ON FALL --------------------------//
        if self.state_buffer.get_status() == Move::Fall {
            let mut speed = 0.0;
            if player.generate_actor().get_position().1 <= -0.8 {
                self.state_buffer.update_status(Move::Walk);
            }
            if time.1 > 0.0 {
                speed = player.get_speed() + self.gravity;
            }
            lst_physical_bodies.push(self.generate_player(&player, speed, -
                (speed * time.1 as f32)));
            //---------------------ON WALK --------------------------//
        } else if self.state_buffer.get_status() == Move::Walk {
            let mut speed = 0.0;
            if keys == "space_press" {
                self.state_buffer.update_status(Move::Jump);
                speed = 1.0;
            }

            let mut p: PhysicalBody = self.generate_player(&player, speed, 0.0);

            //Ajust position, for little stairs maybe
            for o in &lst_physical_bodies {
                if player.get_collision_ray([0.03, -0.01], [0.03, -0.01], o) &&
                    !player.get_collision_ray([0.03, 0.0], [0.03, 0.0], o) {
                    p = self.generate_player(&player, speed, 0.005);
                }
            }

            lst_physical_bodies.push(p);
            //---------------------ON JUMP --------------------------//
        } else if self.state_buffer.get_status() == Move::Jump {
            let speed = player.get_speed() - self.gravity;
            lst_physical_bodies.push(self.generate_player(&player, speed, speed * time.1 as f32));
            if player.get_speed() <= 0.0 {
                self.state_buffer.update_status(Move::Fall);
            }
        }

        for el in &lst_physical_bodies {
            if el.get_name() == "player" &&
                lst_physical_bodies.iter().filter(|x| x.get_name() == "player").count() == 0 {
                result.push(el.generate_actor());
            } else if el.get_name() != " player" {
                result.push(el.generate_actor());
            }
        }

        result.push(Box::new(Text::new("fps".to_string(),
                                       [-0.5, 0.8],
                                       format!("fps : `{fps:.*}`", 2, fps = time.0))));


        (result, lst_physical_bodies)
    }

    fn generate_player(&self, player: &PhysicalBody, speed: f32, position: f32) -> PhysicalBody {
        let el: Box<Actor> = player.generate_actor();

        PhysicalBody::new(player.get_name().to_string(),
                          [el.get_position().0 - 0.025,
                              el.get_position().1 + 0.05],
                          [el.get_position().0 + 0.025,
                              el.get_position().1 - 0.05],
                          Box::new(Actor::new(player.get_name().to_string(),
                                              [-0.8,
                                                  el.get_position().1 + position],
                                              3,
                                              [el.get_size().0,
                                                  el.get_size().1],((0.0,1.0),(1.0,1.0),(1.0,0.0),(0.0,0.0)))),
                          speed)
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
