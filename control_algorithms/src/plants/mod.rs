use crate::linear_algebra::vec3D::Vec3D;

pub trait Plant {
    fn compute(&mut self, ts: f64) -> (Vec3D, Vec3D);
    fn feed(&mut self, u: Vec3D);
}

pub mod chua_circuit;
pub mod lorenz_attractor;
pub mod pendulum;
