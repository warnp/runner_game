use std::cell::RefCell;
use std::rc::{Weak, Rc};
use cgmath::Matrix4;

#[derive(Debug,Clone)]
pub struct Entity{
    pub name: String,
    pub mesh_name: String,
    pub local_matrix: Matrix4<f32>,
    pub matrix: RefCell<Matrix4<f32>>,
    pub children: RefCell<Vec<Rc<RefCell<Entity>>>>,
    pub parent: RefCell<Option<Weak<RefCell<Entity>>>>
}