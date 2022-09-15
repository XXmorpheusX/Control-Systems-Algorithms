use control_algorithms::control_systems::linear_feedback::LinearFeedbackControl;
use control_algorithms::{ControlSimulation, ControlSystem};
use control_algorithms::plants::chua_circuit::chua_circuit::ChuaCircuit;

fn main() {

    let mut system = ControlSystem::new(
        LinearFeedbackControl::new(),
        ChuaCircuit::new(),
    );

    let mut sim = ControlSimulation::new(system, 0.0, 10.0, 0.1);

    loop {
        sim.step();

        if sim.ended() { break; }
    }

    println!("Simulation End.");
}
