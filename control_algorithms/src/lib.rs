use crate::controllers::ControlAlgorithm;
use crate::plants::Plant;
use crate::systems::System;
use linear_algebra::vec::vec3D::Vec3D;

pub mod controllers;
pub mod plants;
pub mod systems;

pub struct ControlSimulation {
    system: Box<dyn System>,
    tf: f64,
    sim_time: f64,
    step_time: f64,
}

impl ControlSimulation {
    pub fn new(system: Box<dyn System>, tf: f64, step_time: f64) -> Self {
        Self {
            system,
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
