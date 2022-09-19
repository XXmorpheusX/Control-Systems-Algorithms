use crate::plants::Plant;
use linear_algebra::vec::vec3D::Vec3D;

pub struct Pendulum {
    pub m: f64,
    pub l: f64,
    pub beta: f64,
    pub j: f64,
    pub k: f64,
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
            j: m * l.powi(2),
            k: 9.81 * m * l,
            x: x0,
            v: v0,
            u: u0 } )
    }
}

impl Plant for Pendulum {
    fn compute(&mut self, ts: f64) -> (Vec3D, Vec3D) {
        // calculations
        let a1 = 2.0 - ts * self.beta / self.j;
        let a2 = ts * self.beta / self.j - 1.0;
        let a3 = -1.0 * ts.powi(2) * self.k / self.j;
        //let b = ts.powi(2) / J;
        println!("{} - {} - {}", a1, a2, a3);

        // Output preparation
        let out_k_2_value = a1 * self.v[1] + a2 * self.x[1] + a3 * f64::sin(self.x[1]);
        let out_k_2 = Vec3D::new(out_k_2_value, 0.0, 0.0);

        let out_v = (self.v - self.x) / ts;
        println!("{} - {} - {}", self.x[1], self.v[1], out_k_2);

        // Updating plant state
        self.x = self.v;
        self.v = out_k_2;

        (out_k_2, out_v)
    }

    fn feed(&mut self, u: Vec3D) {
        self.u = u;
    }
}
