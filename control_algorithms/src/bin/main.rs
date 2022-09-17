use control_algorithms::{ControlSimulation, AutonomousRegularizer};
use control_algorithms::controllers::void_control::VoidControl;
use control_algorithms::linear_algebra::vec3D::Vec3D;
use control_algorithms::plants::chua_circuit::ChuaCircuit;

fn main() {

    let mut system = AutonomousRegularizer::new(
        VoidControl::new(),
        ChuaCircuit::new(
            0.1,
            10.4,
            16.5,
            -1.16,
            0.041,
            Vec3D::new(0.1, 0.2, -0.1),
            Vec3D::new(1.0, -1.0, 0.0)),
    );

    let mut sim = ControlSimulation::new(system, 0.0, 10.0, 0.01);

    loop {
        sim.step();

        if sim.ended() { break; }
    }

    println!("Simulation End.");
}
