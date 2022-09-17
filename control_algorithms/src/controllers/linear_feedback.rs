pub mod linear_feedback {
    use crate::controllers::ControlAlgorithm;
    use crate::linear_algebra::vec3D::Vec3D;

    pub struct LinearFeedbackControl {
        pub xt: Vec3D,
        pub vt: Vec3D,
    }

    impl LinearFeedbackControl {
        pub fn new(xt: Vec3D, vt: Vec3D) -> Box<dyn ControlAlgorithm> {
            Box::new(LinearFeedbackControl { xt, vt })
        }
    }

    impl ControlAlgorithm for LinearFeedbackControl {
        fn compute(&mut self, _x: Vec3D, _v: Vec3D) -> Vec3D {
            Vec3D::new(0.0, 0.0, 0.0)
        }
    }
}