extern crate cgmath;

use self::cgmath::Matrix4;

pub trait GenericCamera {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Matrix4 < f32 >;
    fn get_active(&self) -> bool;
    fn get_aspect(&self) -> f32;
    fn get_view_angle(&self) -> f32;
}