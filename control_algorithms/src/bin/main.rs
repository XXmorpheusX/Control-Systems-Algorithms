use std::thread::sleep;
use std::time::Duration;
use control_algorithms::{ControlSimulation, AutonomousRegularizer};
use control_algorithms::controllers::void_control::VoidControl;
use control_algorithms::linear_algebra::vec3D::Vec3D;
use control_algorithms::plants::chua_circuit::ChuaCircuit;
use serde::{Serialize, Deserialize};
use paho_mqtt as mqtt;
use paho_mqtt::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    x: Vec3D,
    v: Vec3D,
}

fn main() {

    // Mqtt connection
    println!("Setting up mqtt connection...");
    let mut cli = mqtt::AsyncClient::new("tcp://localhost:1883").unwrap();
    let mut tok = cli.connect(mqtt::ConnectOptions::new());
    tok.wait().unwrap();
    println!("Mqtt Works correctly.");

    // System Definition
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
        let (x, v) = sim.step();

        let data = Data { x, v };
        let data_json = serde_json::to_string(&data).unwrap();
        cli.publish(Message::new("CTRL/out", data_json, 2));

        if sim.ended() { break; }
        sleep(Duration::from_millis(5))
    }

    cli.publish(Message::new("CTRL/end", "1", 2));
    println!("Simulation End.");
    sleep(Duration::from_secs(1))
}
