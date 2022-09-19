use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Index, Mul, Sub};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct VecError {
    pub desc: String
}

impl VecError {
    pub fn new(desc: &str) -> Self {
        Self { desc: String::from(desc) }
    }
}

impl Display for VecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong")
    }
}

impl Error for VecError {}

//type Result<T> = std::result::Result<T, VEC_ERROR>;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
                Err(VecError::new("Out of bounds access. Check dimensions.")).unwrap()
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

impl Mul<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Self::Output {
        Vec3D::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<Vec3D> for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: Vec3D) -> Self::Output {
        if rhs.x == 0.0 || rhs.y == 0.0 || rhs.z == 0.0 { Err(VecError::new("Division by Zero. Check vector divisions.")).unwrap() }
        Vec3D::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Add<f64> for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3D::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f64> for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3D::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0.0 { Err(VecError::new("Division by Zero. Check vector divisions.")).unwrap() }
        Vec3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
