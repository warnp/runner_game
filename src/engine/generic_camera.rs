extern crate cgmath;

use self::cgmath::Matrix4;
use std::clone::Clone;

pub trait GenericCamera {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Matrix4 < f32 >;
    fn get_active(&self) -> bool;
    fn get_aspect(&self) -> f32;
    fn get_view_angle(&self) -> f32;
    fn box_clone(&self) -> Box<GenericCamera>;
}

impl Clone for Box<GenericCamera> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
