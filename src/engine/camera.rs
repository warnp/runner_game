pub struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3]
}

impl Camera {
    pub fn new(position: [f32;3], direction: [f32;3], up: [f32;3]) -> Camera {
        Camera {
            position: position,
            direction: direction,
            up: up
        }
    }

    pub fn view_matrix(&self) -> [[f32;4];4] {
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
            let len  = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0]/len, s[1]/len, s[2]/len]
        };

        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
            f[2] * s_norm[0] - f[0] * s_norm[2],
            f[0] * s_norm[1] - f[1] * s_norm[0]];

        let p = [-self.position[0] * s_norm[0] - self.position[1] * s_norm[1] - self.position[2]* s_norm[2],
                                -self.position[0] * u[0] -      self.position[1] * u[1] - self.position[2] * u[2],
                                -self.position[0] * f[0] -       self.position[1]* f[1] - self.position[2] * f[2]];

        [
            [s[0], u[0], f[0], 0.0],
            [s[1], u[1], f[1], 0.0],
            [s[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],

        ]

    }

    pub fn ortho_view(&self) ->[[f32;4];4] {
        [[0.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0],[0.0,0.0,0.0,0.0]]
    }
}