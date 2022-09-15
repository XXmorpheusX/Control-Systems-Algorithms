use control_algorithms::control_systems::linear_feedback::LinearFeedbackControl;
use control_algorithms::{ControlSimulation, AutonomousRegularizer};
use control_algorithms::linear_algebra::vec3D::vec3D::Vec3D;
use control_algorithms::plants::chua_circuit::chua_circuit::ChuaCircuit;

fn main() {

    let mut system = AutonomousRegularizer::new(
        LinearFeedbackControl::new(),
        ChuaCircuit::new(),
        Vec3D::new(0.0, 0.0, 0.0),
        Vec3D::new(1.0, -1.0, 0.0),
        Vec3D::new(5.0, 2.0, 0.0),
        Vec3D::new(0.0, 0.0, 0.0),
    );

    let mut sim = ControlSimulation::new(system, 0.0, 10.0, 0.5);

    loop {
        sim.step();

        if sim.ended() { break; }
    }

    println!("Simulation End.");
}
