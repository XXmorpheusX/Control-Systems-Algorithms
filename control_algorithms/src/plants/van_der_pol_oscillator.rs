use crate::linear_algebra::vec3D::Vec3D;
use crate::plants::Plant;

pub struct VanDerPolOscillator {
    pub mu: f64,
    pub x: Vec3D,
    pub v: Vec3D,
    pub u: Vec3D,
}

impl VanDerPolOscillator {
    pub fn new(mu: f64, x0: Vec3D, v0: Vec3D) -> Box<dyn Plant> {
        let u0 = Vec3D::new(0.0, 0.0, 0.0);
        Box::new( VanDerPolOscillator { mu, x: x0, v: v0, u: u0 } )
    }
}

impl Plant for VanDerPolOscillator {
    fn compute(&mut self, ts: f64) -> (Vec3D, Vec3D) {
        // calculations
        let v1 = self.mu * (self.x[1] - 0.3333 * self.x[1].powi(3) - self.x[2]);
        let v2 = (1.0 / self.mu) * self.x[1];

        // Output preparation
        let out_v = Vec3D::new(v1, v2, 0.0);
        let out_x = self.x + out_v * ts;

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