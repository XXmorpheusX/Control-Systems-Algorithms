#[allow(non_snake_case)]
pub mod vec3D {
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
}
