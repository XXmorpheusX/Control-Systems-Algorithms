use crate::control_systems::ControlAlgorithm;
use crate::plants::Plant;

pub mod linear_algebra;
pub mod control_systems;
pub mod plants;

pub trait System {

}

pub struct ControlSystem {
    algorithm: Box<dyn ControlAlgorithm>,
    plant: Box<dyn Plant>,
}

impl ControlSystem {
    pub fn new(algorithm: Box<dyn ControlAlgorithm>, plant: Box<dyn Plant>) -> Box<dyn System> {
        Box::new(ControlSystem { algorithm, plant })
    }
}

impl System for ControlSystem {

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
        println!(">>> {}", self.sim_time);
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
