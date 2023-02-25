use matrix::matrix::Matrix;
use std::f32::consts::PI;

fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix::<4, 4, f32> {
    let mut result = Matrix::<4, 4, f32>::default();

    let fov_tan = (fov * 0.5 * PI / 180.).tan();
    let half_height = near * fov_tan;
    let half_width = half_height * ratio;

    let left   = -half_width;
    let right  =  half_width;
    let top    =  half_height;
    let bottom = -half_height;

    result[0][0] = (2. * near) / (right - left);
    result[0][2] = (right + left) / (right - left);

    result[1][1] = (2. * near) / (top - bottom);
    result[1][2] = (top + bottom) / (top - bottom);

    result[2][2] = -(far + near) / (far - near);
    result[3][2] = -(2. * far * near) / (far - near);

    result[2][3] = -1.;

    result
}

fn main() {
    for row in projection(60., 1., 0.1, 100.).iter() {
        println!("{}", row.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", "));
    }
}
