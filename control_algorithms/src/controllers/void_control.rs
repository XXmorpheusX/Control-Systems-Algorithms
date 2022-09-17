use crate::controllers::ControlAlgorithm;

pub struct VoidControl {
}

impl VoidControl {
    pub fn new() -> Box<dyn ControlAlgorithm> {
        Box::new(VoidControl {})
    }
}

impl ControlAlgorithm for VoidControl {

}