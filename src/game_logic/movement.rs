pub struct Movements{
    touch_ground: bool,
    start_time: f32,
}

impl Movements {


    pub fn jump(&self, actual_time: f32) -> f32 {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_jump(){
        let moves = Movements::new();
    }
}
