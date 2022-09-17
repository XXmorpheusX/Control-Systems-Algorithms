use control_algorithms::{ControlSimulation, AutonomousRegularizer};
use control_algorithms::controllers::void_control::VoidControl;
use control_algorithms::linear_algebra::vec3D::Vec3D;
use control_algorithms::plants::chua_circuit::ChuaCircuit;

fn main() {

    let mut system = AutonomousRegularizer::new(
        VoidControl::new(),
        ChuaCircuit::new(Vec3D::new(0.0, 0.0, 0.0), Vec3D::new(1.0, -1.0, 0.0)),
    );

    let mut sim = ControlSimulation::new(system, 0.0, 10.0, 0.5);

    loop {
        sim.step();

        if sim.ended() { break; }
    }

    println!("Simulation End.");
}
