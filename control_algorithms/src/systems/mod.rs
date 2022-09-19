use linear_algebra::vec::vec3D::Vec3D;

pub trait System {
    fn step(&mut self, ts: f64) -> (Vec3D, Vec3D);
}

pub mod autonomous_regularizer;