use crate::controllers::ControlAlgorithm;
use crate::linear_algebra::vec3D::Vec3D;
use crate::plants::Plant;

pub mod linear_algebra;
pub mod controllers;
pub mod plants;

pub trait System {
    fn step(&mut self, ts: f64) -> (Vec3D, Vec3D);
}

pub struct AutonomousRegularizer {
    algorithm: Box<dyn ControlAlgorithm>,
    plant: Box<dyn Plant>,
}

impl AutonomousRegularizer {
    pub fn new(algorithm: Box<dyn ControlAlgorithm>, plant: Box<dyn Plant>) -> Box<dyn System> {
        Box::new(AutonomousRegularizer {
            algorithm,
            plant,
        })
    }
}

impl System for AutonomousRegularizer {
    fn step(&mut self, ts: f64) -> (Vec3D, Vec3D) {
        let (x, v) = self.plant.compute(ts);
        let u = self.algorithm.compute(x, v);
        self.plant.feed(u);
        (x, v)
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

    pub fn step(&mut self) -> (Vec3D, Vec3D) {
        //println!(">>> {:.2}", self.sim_time);
        let res = self.system.step(self.step_time);
        //println!("---------------");
        println!("{:.2}%", 100.0 * self.sim_time / self.tf);

        self.sim_time += self.step_time;
        res
    }

    pub fn ended(&self) -> bool {
        self.sim_time > self.tf
    }
}
