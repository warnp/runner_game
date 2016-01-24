pub struct Movements{
    touch_ground: bool,
    start_time: f32,
    max_height: f32,
}

impl Movements {
    pub fn new(max_height) -> Movements {
        Movements {
            touch_ground: true,
            start_time: 0.0,
            max_height: max_height,
        }
    }

    pub fn jump(&mut self, actual_time: f32) -> f32 {
        if self.touch_ground {
            self.start_time = actual_time;
            1.0
        }

        30.0/actual_time - start_time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_reach_highest_point(){
        let moves = Movements::new(0.5);
        let mut height = 0.0;
        let mut reach_top = false;
        for i in 0..120 {
            height = moves.jump(i);
            if height == 0.5 {
                reach_top = true;
            }
        }

        assert!(reach_top);
    }

    #[test]
    fn should_not_jump(){
        let moves = Movements::new(0.5);
        let mut height = 0.0;
        let mut reach_top = false;
        for i in 0..120 {
            height = moves.jump(i);
            if height == 0.5 {
                reach_top = true;
            }
        }


        assert!(!moves.touch_ground());
    }

    #[test]
    fn should_jump(){
        let moves = Movements::new(0.5);
        let mut height = 0.0;
        let mut reach_top = false;
        for i in 0..120 {
            height = moves.jump(i);
            if height == 0.5 {
                reach_top = true;
            }
        }


        assert!(moves.touch_ground());
    }


}
