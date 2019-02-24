//! maths utility functions
//! 
pub use std::f32::consts::PI;

pub type Point2 = nalgebra::Point2<f32>;
pub type Vector2 = nalgebra::Vector2<f32>;

/// ensure angle is in range [-PI, Pi]
pub fn angle_clamp( angle: f32 )->f32{

    let mut new_angle = angle;
    while new_angle > PI {
        new_angle -= PI * 2.0;
    }

    while new_angle < -PI {
        new_angle += PI * 2.0;
    } 
    new_angle
}

/// Create a unit vector representing the given angle (in radians)
pub fn vector_from_angle( angle: f32)->Vector2
{
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}