extern crate cgmath;

use self::cgmath::{Matrix4, Vector3, Point3};
use self::cgmath::prelude::*;
use self::cgmath::{perspective, ortho};
use self::cgmath::{Deg, Rad};
use std::ops::Mul;


pub struct Camera {
    pub name: String,
    pub position: Matrix4<f32>,
    pub rotation: Matrix4<f32>,
    pub active: bool,
    pub aspect: f32,
    pub view_angle: f32,
}

impl Camera {
    pub fn look_at(view_angle: f32, aspect: f32, near: f32, far: f32, location: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
        let proj = perspective(Deg(view_angle), aspect, near, far);

        let camera_matrix = Matrix4::look_at(location, target, up);
        let view_matrix = camera_matrix.invert().unwrap();
        proj.mul(view_matrix)
    }

    pub fn set_position(&mut self, translation: Vector3<f32>) {
        self.position = Matrix4::from_translation(translation);
    }

    pub fn set_rotation(&mut self, roll: f32, pitch: f32, yaw: f32) {
        let mat_roll = Matrix4::from_angle_z(Rad(roll));
        let mat_pitch = Matrix4::from_angle_x(Rad(pitch));
        let mat_yaw = Matrix4::from_angle_y(Rad(yaw));

        self.rotation = mat_roll.mul(mat_pitch).mul(mat_yaw);
    }

    pub fn generate_rotation(roll: f32, pitch: f32, yaw: f32) -> Matrix4<f32> {
        let mat_roll = Matrix4::from_angle_z(Rad(roll));
        let mat_pitch = Matrix4::from_angle_x(Rad(pitch));
        let mat_yaw = Matrix4::from_angle_y(Rad(yaw));

        mat_roll.mul(mat_pitch).mul(mat_yaw)
    }

    pub fn fps(&self, near: f32, far: f32) -> Matrix4<f32> {
        let proj = perspective(Deg(self.view_angle), self.aspect, near, far);
//        let w = 100.0;
//        let proj = ortho(-w,w,-w,w, near, far);

        let camera_matrix = self.rotation.mul(self.position);
        let camera_matrix = camera_matrix.invert().unwrap();
        let view_matrix = proj.mul(camera_matrix);

        view_matrix
    }
}