use crate::control_systems::ControlAlgorithm;

pub struct LinearFeedbackControl {
}

impl LinearFeedbackControl {
    pub fn new() -> Box<dyn ControlAlgorithm> {
        Box::new(LinearFeedbackControl {})
    }
}

impl ControlAlgorithm for LinearFeedbackControl {

}
