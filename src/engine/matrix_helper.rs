pub struct MatrixHelper;

impl MatrixHelper {
    pub fn identity() -> [[f32;4];4]{
        [[1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]]
    }

    pub fn rotate_x(angle:f32) -> [[f32;4];4]{
        [[1.0,0.0        ,0.0         ,0.0],
         [0.0,angle.cos(),-angle.sin(),0.0],
         [0.0,angle.sin(),angle.cos() ,0.0],
         [0.0,0.0        ,0.0         ,1.0]]
    }

    pub fn rotate_y(angle:f32) -> [[f32;4];4]{
        [[angle.cos() ,0.0,angle.sin(),0.0],
         [0.0         ,1.0,0.0        ,0.0],
         [-angle.sin(),0.0,angle.cos(),0.0],
         [0.0         ,0.0,0.0        ,1.0]]
    }

    pub fn rotate_z(angle:f32) -> [[f32;4];4]{
        [[angle.cos(),-angle.sin(),0.0,0.0],
         [angle.sin(),angle.cos() ,0.0,0.0],
         [0.0        ,0.0         ,1.0,0.0],
         [0.0        ,0.0         ,0.0,1.0]]
    }

    pub fn translate(x:f32,y:f32,z:f32) -> [[f32;4];4]{
        [[1.0,0.0,0.0,0.0],
         [0.0,1.0,0.0,0.0],
         [0.0,0.0,1.0,0.0],
         [ x , y , z ,1.0]]
    }

    pub fn scale(x:f32,y:f32,z:f32) -> [[f32;4];4]{
        [[ x ,0.0,0.0,0.0],
         [0.0, y ,0.0,0.0],
         [0.0,0.0, z ,0.0],
         [0.0,0.0,0.0,1.0]]
    }

    pub fn mult4(a:[[f32;4];4],b:[[f32;4];4]) -> [[f32;4];4] {

        let a00 = a[0][0];
        let a01 = a[0][1];
        let a02 = a[0][2];
        let a03 = a[0][3];
        let a10 = a[1][0];
        let a11 = a[1][1];
        let a12 = a[1][2];
        let a13 = a[1][3];
        let a20 = a[2][0];
        let a21 = a[2][1];
        let a22 = a[2][2];
        let a23 = a[2][3];
        let a30 = a[3][0];
        let a31 = a[3][1];
        let a32 = a[3][2];
        let a33 = a[3][3];

        let b00 = b[0][0];
        let b01 = b[0][1];
        let b02 = b[0][2];
        let b03 = b[0][3];
        let b10 = b[1][0];
        let b11 = b[1][1];
        let b12 = b[1][2];
        let b13 = b[1][3];
        let b20 = b[2][0];
        let b21 = b[2][1];
        let b22 = b[2][2];
        let b23 = b[2][3];
        let b30 = b[3][0];
        let b31 = b[3][1];
        let b32 = b[3][2];
        let b33 = b[3][3];


        let result =[[a00*b00+a01*b10+a02*b20+a03*b30, a00*b01+a01*b11+a02*b21+a03*b31, a00*b02+a01*b12+a02*b22+a03*b32, a00*b03+a01*b13+a02*b23+a03*b33],
        [a10*b00+a11*b10+a12*b20+a13*b30, a10*b01+a11*b11+a12*b21+a13*b31, a10*b02+a11*b12+a12*b22+a13*b32, a10*b03+a11*b13+a12*b23+a13*b33],
        [a20*b00+a21*b10+a22*b20+a23*b30, a20*b01+a21*b11+a22*b21+a23*b31, a20*b02+a21*b12+a22*b22+a23*b32, a20*b03+a21*b13+a22*b23+a23*b33],
        [a30*b00+a31*b10+a32*b20+a33*b30, a30*b01+a31*b11+a32*b21+a33*b31, a30*b32+a31*b12+a32*b22+a33*b32, a30*b03+a31*b13+a32*b23+a33*b33] ];



        result
    }

    pub fn cross(a:[f32;3], b:[f32;3]) -> [f32;3] {
        [a[1]*b[2] - a[2]*b[1],
         a[2]*b[0] - a[0]*b[2],
         a[0]*b[1] - a[1]*b[0]]
    }

    pub fn substract(a:[f32;3],b:[f32;3]) -> [f32;3] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
    }

    pub fn normalize(a:[f32;3])->[f32;3]{
        let norme = {a[0]*a[0] + a[1]*a[1] + a[2]*a[2]}.sqrt();
        if norme > 0.00001 {
            [a[0] / norme, a[1] / norme, a[2] / norme]
        } else {
            [0.0, 0.0, 0.0]
        }
    }

    pub fn look_at(camera_pos:[f32;3], target:[f32;3],up:[f32;3]) -> [[f32;4];4]{

        let axe_z = MatrixHelper::normalize(MatrixHelper::substract(camera_pos, target));
        let axe_x = MatrixHelper::cross(up, axe_z);
        let axe_y = MatrixHelper::cross(axe_z, axe_x);

        let result = [[axe_x[0]     ,axe_x[1]     ,axe_x[2]     ,0.0],
         [axe_y[0]     ,axe_y[1]     ,axe_y[2]     ,0.0],
         [axe_z[0]     ,axe_z[1]     ,axe_z[2]     ,0.0],
         [camera_pos[0],camera_pos[1],camera_pos[2],1.0]];

        result
    }

    // NOTE: this is an extremely efficient, loop-unrolled matrix inverse from MESA (MIT licenced). Same algo used by Nalgebra
    pub fn inverse(m:[[f32;4];4]) -> Option<[[f32;4];4]> {

        let mut out = [[0.0,0.0,0.0,0.0],
                        [0.0,0.0,0.0,0.0],
                        [0.0,0.0,0.0,0.0],
                        [0.0,0.0,0.0,0.0],];

        out[0][0] = m[1][1]  * m[2][2] * m[3][3] -
                    m[1][1]  * m[2][2] * m[3][2] -
                    m[2][0]  * m[1][2]  * m[3][3] +
                    m[2][1]  * m[1][3]  * m[3][2] +
                    m[3][1] * m[1][2]  * m[2][3] -
                    m[3][1] * m[1][3]  * m[2][2];

        out[1][0] = -m[0][1]  * m[2][2] * m[3][3] +
                    m[0][1]  * m[2][3] * m[3][2] +
                    m[2][1]  * m[0][2] * m[3][3] -
                    m[2][1]  * m[0][3] * m[3][2] -
                    m[3][1] * m[0][2] * m[2][3] +
                    m[3][1] * m[0][3] * m[2][2];

        out[2] [0]= m[0][1]  * m[1][2] * m[3][3] -
                    m[0][1]  * m[1][3] * m[3][2] -
                    m[1][1]  * m[0][2] * m[3][3] +
                    m[1][1]  * m[0][3] * m[3][2] +
                    m[3][1] * m[0][2] * m[1][3] -
                    m[3][1] * m[0][3] * m[1][2];

        out[3][0]= -m[0][1] * m[1][2] * m[2][3] +
                    m[0][1] * m[1][3] * m[2][2] +
                    m[1][1] * m[0][2] * m[2][3] -
                    m[1][1] * m[0][3] * m[2][2] -
                    m[2][1] * m[0][2] * m[1][3] +
                    m[2][1] * m[0][3] * m[1][2];

        out[0][1]= -m[1][0]  * m[2][2] * m[3][3] +
                    m[1][0]  * m[2][3] * m[3][2] +
                    m[2][0]  * m[1][2]  * m[3][3] -
                    m[2][0]  * m[1][3]  * m[3][2] -
                    m[3][0] * m[1][2]  * m[2][3] +
                    m[3][0] * m[1][3]  * m[2][2];

        out[1][1] = m[0][0]  * m[2][2] * m[3][3] -
                    m[0][0]  * m[2][3] * m[3][2] -
                    m[2][0]  * m[0][2] * m[3][3] +
                    m[2][0]  * m[0][3] * m[3][2] +
                    m[3][0] * m[0][2] * m[2][3] -
                    m[3][0] * m[0][3] * m[2][2];

        out[2][1] = -m[0][0]  * m[1][2] * m[3][3] +
                    m[0][0]  * m[1][3] * m[3][2] +
                    m[1][0]  * m[0][2] * m[3][3] -
                    m[1][0]  * m[0][3] * m[3][2] -
                    m[3][0] * m[0][2] * m[1][3] +
                    m[3][0] * m[0][3] * m[1][2];

        out[3][1] = m[0][0] * m[1][2] * m[2][3] -
                    m[0][0] * m[1][3] * m[2][2] -
                    m[1][0] * m[0][2] * m[2][3] +
                    m[1][0] * m[0][3] * m[2][2] +
                    m[2][0] * m[0][2] * m[1][3] -
                    m[2][0] * m[0][3] * m[1][2];

        out[0][2] = m[1][0]  * m[2][1] * m[3][3] -
                    m[1][0]  * m[2][3] * m[3][1] -
                    m[2][0]  * m[1][1] * m[3][3] +
                    m[2][0]  * m[1][3] * m[3][1] +
                    m[3][0] * m[1][1] * m[2][3] -
                    m[3][0] * m[1][3] * m[2][1];

        out[1][2] = -m[0][0]  * m[2][1] * m[3][3] +
                    m[0][0]  * m[2][3] * m[3][1] +
                    m[2][0]  * m[0][1] * m[3][3] -
                    m[2][0]  * m[0][3] * m[3][1] -
                    m[3][0] * m[0][1] * m[2][3] +
                    m[3][0] * m[0][3] * m[2][1];

        out[2][2] = m[0][0]  * m[1][1] * m[3][3] -
                    m[0][0]  * m[1][3] * m[3][1] -
                    m[1][0]  * m[0][1] * m[3][3] +
                    m[1][0]  * m[0][3] * m[3][1] +
                    m[3][0] * m[0][1] * m[1][3] -
                    m[3][0] * m[0][3] * m[1][1];

        out[0][3] = -m[1][0]  * m[2][1] * m[3][2] +
                    m[1][0]  * m[2][2] * m[3][1] +
                    m[2][0]  * m[1][1] * m[3][2] -
                    m[2][0]  * m[1][2] * m[3][1] -
                    m[3][0] * m[1][1] * m[2][2] +
                    m[3][0] * m[1][2] * m[2][1];

        out[3][2] = -m[0][0] * m[1][1] * m[2][3] +
                    m[0][0] * m[1][3] * m[2][1] +
                    m[1][0] * m[0][1] * m[2][3] -
                    m[1][0] * m[0][3] * m[2][1] -
                    m[2][0] * m[0][1] * m[1][3] +
                    m[2][0] * m[0][3] * m[1][1];

        out[1][3] = m[0][0]  * m[2][1] * m[3][2] -
                    m[0][0]  * m[2][2] * m[3][1] -
                    m[2][0]  * m[0][1] * m[3][2] +
                    m[2][0]  * m[0][2] * m[3][1] +
                    m[3][0] * m[0][1] * m[2][2] -
                    m[3][0] * m[0][2] * m[2][1];

        out[2][3] = -m[0][0]  * m[1][1] * m[3][2] +
                    m[0][0]  * m[1][2] * m[3][1] +
                    m[1][0]  * m[0][1] * m[3][2] -
                    m[1][0]  * m[0][2] * m[3][1] -
                    m[3][0] * m[0][1] * m[1][2] +
                    m[3][0] * m[0][2] * m[1][1];

        out[3][3] = m[0][0] * m[1][1] * m[2][2] -
                    m[0][0] * m[1][2] * m[2][1] -
                    m[1][0] * m[0][1] * m[2][2] +
                    m[1][0] * m[0][2] * m[2][1] +
                    m[2][0] * m[0][1] * m[1][2] -
                    m[2][0] * m[0][2] * m[1][1];

        let det = m[0][0] * out[0][0] + m[0][1] * out[0][1] + m[0][2] * out[0][2] + m[0][3] * out[0][3];
        if det != 0.0 {
            let inv_det = 1.0 / det;

            for j in 0 .. 4 {
                for i in 0 .. 4 {
                    out[i][j] *= inv_det;
                }
            }

            Some(out)
        }else {
            None
        }

    }

    pub fn perspective(fov_radians:f32, aspect: f32, near:f32,far:f32) -> [[f32;4];4] {
        let f = 1.0 / (fov_radians / 2.0).tan();  //(3.141592 * 0.5 - 0.5 * ).tan();

        let range_inv = 1.0 / (near - far);

        [[f/aspect,0.0 ,0.0             ,0.0],
        [0.0      ,f   ,0.0             ,0.0],
        [0.0      ,0.0 ,(near+far)/(far - near),1.0],
        [0.0      ,0.0 ,-(2.0*far*near)/(far-near),0.0]]
    }
}