use crate::controllers::ControlAlgorithm;
use linear_algebra::vec::vec3D::Vec3D;

pub struct VoidControl {
}

impl VoidControl {
    pub fn new() -> Box<dyn ControlAlgorithm> {
        Box::new(VoidControl {})
    }
}

impl ControlAlgorithm for VoidControl {
    fn compute(&mut self,  _x: Vec3D, _v: Vec3D) -> Vec3D {
        Vec3D::new(0.0, 0.0, 0.0)
    }
}