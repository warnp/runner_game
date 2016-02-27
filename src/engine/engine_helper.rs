extern crate time;

pub struct EngineHelper {
    old_time: f64,
    iterator: u64,
}

impl EngineHelper {
    pub fn new() -> EngineHelper {
        EngineHelper {
            old_time: 0.0,
            iterator: 0u64,
        }
    }

    pub fn get_fps(&mut self) -> (f64, f64) {
        let time = time::precise_time_ns() as f64;
        self.iterator = self.iterator + 1u64;
        let mut time_between = 0.0f64;
        if self.iterator > 60u64 {
            time_between = time / 1000000000.0 - self.old_time / 1000000000.0;
        }


        let fps = 1.0f64 / time_between;
        self.old_time = time;

        (fps, time_between)
    }

    pub fn get_iterator(&self) -> u64 {
        self.iterator
    }
}

#[cfg(test)]
mod engine_helper_tests {

    use super::*;

    #[test]
    fn should_get_fps() {
        let mut engine_helper = EngineHelper::new();

        for x in 0..120 {
            if x > 60 {
                assert!(engine_helper.get_fps().0 > 0.0);
            }
        }

    }

    #[test]
    fn should_return_iterator() {
        let mut engine_helper = EngineHelper::new();

        for x in 0..120 {

            engine_helper.get_fps();

        }
        println!("{:?}", engine_helper.get_iterator());
        assert!(engine_helper.get_iterator() == 120.0);
    }
}
