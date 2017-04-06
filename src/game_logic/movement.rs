
pub trait State {
    fn go_next(movements: &mut Movements, new_status: Move)-> Move;
}

pub struct Fall ;

pub struct Walk ;

pub struct Jump ;

impl State for Fall {
    fn go_next(movements: &mut Movements, new_status: Move) -> Move {
        match new_status {
            Move::Walk => {movements.set_status(new_status); movements.get_status()},
            Move::Jump => {movements.get_status()},
            Move::Fall => {movements.get_status()},
        }

    }
}

impl State for Jump {
    fn go_next(movements: &mut Movements, new_status: Move) -> Move{

        match new_status {
            Move::Fall => {movements.set_status(new_status); movements.get_status()},
            Move::Jump => {movements.get_status()},
            Move::Walk => {movements.get_status()},
        }
    }
}

impl State for Walk {
    fn go_next(movements: &mut Movements, new_status: Move) -> Move{
        match new_status {
            Move::Jump => {movements.set_status(new_status); movements.get_status()},
            Move::Fall => {movements.set_status(new_status); movements.get_status()},
            Move::Walk => {movements.get_status()},
        }

    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Move {
    Fall,
    Walk,
    Jump,
}

#[derive(Debug, Clone)]
pub struct Movements{
    state: Move,
}

impl Movements {
    pub fn new() -> Movements {
        Movements {
            state: Move::Fall,
        }
    }

    pub fn get_status(&self) -> Move {
        self.state.clone()
    }

    fn set_status(&mut self, new_status: Move) {
        self.state = new_status;
    }

    pub fn update_status(&mut self, new_status: Move){
        match self.state {
            Move::Jump => {Jump::go_next(self, new_status);},
            Move::Fall => {Fall::go_next(self, new_status);},
            Move::Walk => {Walk::go_next(self, new_status);},
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_actual_status(){
        let mut moves = Movements::new();

        assert!(moves.get_status() == Move::Fall);
    }

    #[test]
    fn should_go_from_walk_to_jump(){
        let mut moves = Movements::new();
        moves.update_status(Move::Walk);

        moves.update_status(Move::Jump);

        assert!(moves.get_status() == Move::Jump);
    }

    #[test]
    fn should_go_from_walk_to_fall(){
        let mut moves = Movements::new();
        moves.update_status(Move::Walk);

        moves.update_status(Move::Fall);

        assert!(moves.get_status() == Move::Fall);
    }


    #[test]
    fn should_go_from_jump_to_fall(){
        let mut moves = Movements::new();

        moves.update_status(Move::Jump);

        moves.update_status(Move::Fall);

        assert!(moves.get_status() == Move::Fall);
    }





    #[test]
    fn should_go_from_fall_to_walk(){
        let mut moves = Movements::new();
        // moves.update_status(Move::Fall);

        moves.update_status(Move::Walk);
        assert!(moves.get_status() == Move::Walk);
    }

    #[test]
    fn should_not_go_from_fall_to_jump(){
        let mut moves = Movements::new();
        // moves.update_status(Move::Fall);
        moves.update_status(Move::Jump);

        assert!(moves.get_status() == Move::Fall);
    }


}
