use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Index, Mul, Sub};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct VEC_ERROR {
    pub desc: String
}

impl VEC_ERROR {
    pub fn new(desc: &str) -> Self {
        Self { desc: String::from(desc) }
    }
}

impl Display for VEC_ERROR {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong")
    }
}

impl Error for VEC_ERROR {}

type Result<T> = std::result::Result<T, VEC_ERROR>;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
        write!(f, "{:.2} {:.2} {:.2}", self.x, self.y, self.z)
    }
}

impl Index<usize> for Vec3D {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            1 => { &self.x },
            2 => { &self.y },
            3 => { &self.z }
            _ => {
                Err(VEC_ERROR::new("Out of bounds access. Check dimensions.")).unwrap()
            }
        }
    }
}

impl Add<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Self::Output {
        Vec3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Vec3D) -> Self::Output {
        Vec3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
