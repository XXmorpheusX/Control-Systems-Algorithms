use crate::Vec3D;

pub trait ControlAlgorithm {
    fn compute(&mut self, x: Vec3D, v: Vec3D) -> Vec3D;
}

pub mod void_control;
pub mod linear_feedback;
