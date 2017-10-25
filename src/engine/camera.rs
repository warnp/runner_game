extern crate cgmath;

use self::cgmath::{Matrix4, Vector3, Point3};
use self::cgmath::prelude::*;
use self::cgmath::conv::*;
use self::cgmath::perspective;
use self::cgmath::PerspectiveFov;
use self::cgmath::{Deg, Rad};
use std::ops::Mul;


pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3]
}

impl Camera {
    pub fn new(position: [f32; 3], direction: [f32; 3], up: [f32; 3]) -> Camera {
        Camera {
            position: position,
            direction: direction,
            up: up
        }
    }

    pub fn first_person(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let s = [self.up[1] * f[2] - self.up[2] * f[1],
            self.up[2] * f[0] - self.up[0] * f[2],
            self.up[0] * f[1] - self.up[1] * f[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0]];

        let p = [-self.position[0] * s_norm[0] - self.position[1] * s_norm[1] - self.position[2] * s_norm[2],
            -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
            -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2]];

        [
            [s[0], u[0], f[0], 0.0],
            [s[1], u[1], f[1], 0.0],
            [s[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    pub fn ortho_view(&self) -> [[f32; 4]; 4] {
        [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]]
    }

    pub fn look_at(view_angle: f32, aspect: f32, near: f32, far: f32, location: Point3<f32>, target: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
        let proj = perspective(Deg(view_angle), aspect, near, far);

        let camera_matrix = Matrix4::look_at(location, target, up);
        let view_matrix = camera_matrix.invert().unwrap();
        proj.mul(view_matrix)
    }

    pub fn fps(view_angle: f32, aspect: f32, near: f32, far: f32, roll: f32, pitch: f32, yaw: f32, translation: Vector3<f32>) -> Matrix4<f32> {
        let proj = perspective(Deg(view_angle), aspect, near, far);

        let mat_roll = Matrix4::from_angle_z(Rad(roll));
        let mat_pitch = Matrix4::from_angle_x(Rad(pitch));
        let mat_yaw = Matrix4::from_angle_y(Rad(yaw));

        let rotate = mat_roll.mul(mat_pitch).mul(mat_yaw);
        let mat_position = Matrix4::from_translation(translation);

        let camera_matrix = rotate.mul(mat_position);
        let camera_matrix = camera_matrix.invert().unwrap();
        let view_matrix = proj.mul(camera_matrix);

        view_matrix
    }
}