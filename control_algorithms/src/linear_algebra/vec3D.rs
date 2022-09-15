#[allow(non_snake_case)]
pub mod vec3D {
    use std::fmt::{Display, Formatter};

    #[derive(Debug, Clone, Copy)]
    pub struct Vec3D {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vec3D {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }
    }

    impl Display for Vec3D {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{:.2} {:.2} {:.2}]", self.x, self.y, self.z)
        }
    }
}
