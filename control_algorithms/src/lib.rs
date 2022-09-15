use crate::control_systems::ControlAlgorithm;
use crate::linear_algebra::vec3D::vec3D::Vec3D;
use crate::plants::Plant;

pub mod linear_algebra;
pub mod control_systems;
pub mod plants;

pub trait System {
    fn step(&mut self);
}

pub struct AutonomousRegularizer {
    algorithm: Box<dyn ControlAlgorithm>,
    plant: Box<dyn Plant>,
    x0: Vec3D,
    v0: Vec3D,
    xt: Vec3D,
    vt: Vec3D,
    x: Vec3D,
    v: Vec3D,
}

impl AutonomousRegularizer {
    pub fn new(algorithm: Box<dyn ControlAlgorithm>, plant: Box<dyn Plant>, x0: Vec3D, v0: Vec3D, xt: Vec3D, vt: Vec3D) -> Box<dyn System> {
        Box::new(AutonomousRegularizer {
            algorithm,
            plant,
            x0,
            v0,
            xt,
            vt,
            x: x0,
            v: v0,
        })
    }
}

impl System for AutonomousRegularizer {
    fn step(&mut self) {
        println!("x: {}, v: {}", self.x, self.v);
    }
}

pub struct ControlSimulation {
    system: Box<dyn System>,
    t0: f64,
    tf: f64,
    sim_time: f64,
    step_time: f64,
}

impl ControlSimulation {
    pub fn new(system: Box<dyn System>, t0: f64, tf: f64, step_time: f64) -> Self {
        Self {
            system,
            t0,
            tf,
            sim_time: 0.0,
            step_time,
        }
    }

    pub fn step(&mut self) {
        println!(">>> {:.2}", self.sim_time);
        self.system.step();
        println!("---------------");

        self.sim_time += self.step_time;
    }

    pub fn ended(&self) -> bool {
        if self.sim_time > self.tf {
            true
        } else {
            false
        }
    }
}
