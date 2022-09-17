use crate::linear_algebra::vec3D::Vec3D;
use crate::plants::Plant;

pub struct Pendulum {
    pub m: f64,
    pub l: f64,
    pub beta: f64,
    pub J: f64,
    pub K: f64,
    pub x: Vec3D,
    pub v: Vec3D,
    pub u: Vec3D,
}

impl Pendulum {
    pub fn new(m: f64, l: f64, beta: f64, x0: Vec3D, v0: Vec3D) -> Box<dyn Plant> {
        let u0 = Vec3D::new(0.0, 0.0, 0.0);
        Box::new( Pendulum {
            m,
            l,
            beta,
            J: m * l.powi(2),
            K: 9.81 * m * l,
            x: x0,
            v: v0,
            u: u0 } )
    }
}

impl Plant for Pendulum {
    fn compute(&mut self, ts: f64) -> (Vec3D, Vec3D) {
        // calculations
        let a1 = 2.0 - ts * self.beta / self.J;
        let a2 = ts * self.beta / self.J - 1.0;
        let a3 = -1.0 * ts.powi(2) * self.K / self.J;
        //let b = ts.powi(2) / J;

        // Output preparation
        let out_k_2_value = a1 * self.v[1] + a2 * self.x[1] + a3 * f64::sin(self.x[1]);
        let out_k_2 = Vec3D::new(out_k_2_value, 0.0, 0.0);

        let out_v = out_k_2 - self.v;

        // Updating plant state
        self.x = self.v;
        self.v = out_k_2;

        (out_k_2, out_v)
    }

    fn feed(&mut self, u: Vec3D) {
        self.u = u;
        //println!("Updating input u to : {}", self.u);
    }
}
