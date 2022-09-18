use crate::linear_algebra::vec3D::Vec3D;
use crate::plants::Plant;

pub struct LorenzAttractor {
    pub sigma: f64,
    pub beta: f64,
    pub rho: f64,
    pub x: Vec3D,
    pub v: Vec3D,
    pub u: Vec3D,
}

impl LorenzAttractor {
    pub fn new(sigma: f64, beta: f64, rho: f64, x0: Vec3D, v0: Vec3D) -> Box<dyn Plant> {
        let u0 = Vec3D::new(0.0, 0.0, 0.0);
        Box::new( LorenzAttractor { sigma, beta, rho, x: x0, v: v0, u: u0 } )
    }
}

impl Plant for LorenzAttractor {
    fn compute(&mut self, ts: f64) -> (Vec3D, Vec3D) {
        // calculations
        //let v1 = self.sigma * (self.x[2] - self.x[1]);
        //let v2 = self.x[1] * (self.rho + self.x[3]) - self.x[2];
        //let v3 = self.x[1] * self.x[2] - self.beta * self.x[3];
        let x1 = self.x[1] + self.sigma * (self.x[2] - self.x[1]) * ts;
        let x2 = self.x[2] + (self.rho * self.x[1] - self.x[2] - self.x[1] * self.x[3]) * ts;
        let x3 = self.x[3] + (self.x[1] * self.x[2] - self.beta * self.x[3]) * ts;

        // Output preparation
        //let mut out_v = Vec3D::new(v1, v2, v3);
        //let mut out_x = self.x + out_v * ts;
        let mut out_v = Vec3D::new(0.0, 0.0, 0.0);
        let mut out_x = Vec3D::new(x1, x2, x3);

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