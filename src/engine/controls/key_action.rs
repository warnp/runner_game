use engine::graphic::generic_camera::GenericCamera;

pub trait AnyKeyAction {
    fn execute_action(&self, Vec<Box<GenericCamera>>) -> Vec<Box<GenericCamera>>;
}

#[derive(Clone)]
pub struct KeyAction<F : Fn(Vec<Box<GenericCamera>>) -> Vec<Box<GenericCamera>>> {
    pub key: String,
    pub action: F,
}

impl <F> AnyKeyAction for KeyAction<F> where F: Fn(Vec<Box<GenericCamera>>) -> Vec<Box<GenericCamera>> {
    fn execute_action(&self,controls_camera: Vec<Box<GenericCamera>>) -> Vec<Box<GenericCamera>>{
        (self.action)(controls_camera)
    }
}