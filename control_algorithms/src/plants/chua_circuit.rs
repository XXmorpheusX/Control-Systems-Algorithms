use crate::linear_algebra::vec3D::Vec3D;
use crate::plants::Plant;

pub struct ChuaCircuit {
    pub R: f64,
    pub alpha: f64,
    pub beta: f64,
    pub c: f64,
    pub d: f64,
    pub x: Vec3D,
    pub v: Vec3D,
    pub u: Vec3D,
}

impl ChuaCircuit {
    pub fn new(R: f64, alpha: f64, beta: f64, c: f64, d: f64, x0: Vec3D, v0: Vec3D) -> Box<dyn Plant> {
        let u0 = Vec3D::new(0.0, 0.0, 0.0);
        Box::new( ChuaCircuit { R, alpha, beta, c, d, x: x0, v: v0, u: u0 } )
    }
}

impl Plant for ChuaCircuit {
    fn compute(&mut self, ts: f64) -> (Vec3D, Vec3D) {
        // calculations
        let rho = self.c * self.x[1] + self.d * self.x[1].powi(3);
        let v1 = self.alpha * (self.x[2] - self.x[1] - rho);
        let v2 = self.x[1] - self.x[2] + self.x[3] + self.u[1];
        let v3 = -self.beta * self.x[2] - self.R * self.x[3];

        // Output preparation
        let mut out_v = Vec3D::new(v1, v2, v3);
        let mut out_x = self.x + out_v * ts;
        println!("{} {}", self.x, self.v);
        //println!("Current State : {} - {}  |  Input : {}", self.x, self.v, self.u);
        //println!("Output: {} - {}", out_x, out_v);

        // Updating plant state
        self.x = out_x;
        self.v = out_v;

        (out_x, out_v)
    }

    fn feed(&mut self, u: Vec3D) {
        self.u = u;
        //println!("Updating input u to : {}", self.u);
    }
}
