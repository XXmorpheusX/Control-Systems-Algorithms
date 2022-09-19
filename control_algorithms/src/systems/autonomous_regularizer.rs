use crate::{ControlAlgorithm, Plant, System};
use linear_algebra::vec::vec3D::Vec3D;

pub struct AutonomousRegularizer {
    algorithm: Box<dyn ControlAlgorithm>,
    plant: Box<dyn Plant>,
}

impl AutonomousRegularizer {
    pub fn new(algorithm: Box<dyn ControlAlgorithm>, plant: Box<dyn Plant>) -> Box<dyn System> {
        Box::new(AutonomousRegularizer {
            algorithm,
            plant,
        })
    }
}

impl System for AutonomousRegularizer {
    fn step(&mut self, ts: f64) -> (Vec3D, Vec3D) {
        let (x, v) = self.plant.compute(ts);
        let u = self.algorithm.compute(x, v);
        self.plant.feed(u);
        (x, v)
    }
}