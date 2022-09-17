use crate::linear_algebra::vec3D::Vec3D;
use crate::plants::Plant;

pub struct ChuaCircuit {
    pub x: Vec3D,
    pub v: Vec3D,
    pub u: Vec3D,
}

impl ChuaCircuit {
    pub fn new(x0: Vec3D, v0: Vec3D) -> Box<dyn Plant> {
        let u0 = Vec3D::new(0.0, 0.0, 0.0);
        Box::new( ChuaCircuit { x: x0, v: v0, u: u0 } )
    }
}

impl Plant for ChuaCircuit {
    fn compute(&mut self) {
        let mut output = Vec3D::new(0.0, 0.0, 0.0);
        println!("Current State : {} - {}  |  Input : {}", self.x, self.v, self.u);
        println!("Output: {}", output);
    }

    fn feed(&mut self, u: Vec3D) {
        self.u = u;
        println!("Updating input u to : {}", self.u);
    }
}
