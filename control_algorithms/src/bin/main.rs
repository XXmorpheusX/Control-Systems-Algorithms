use std::thread::sleep;
use std::time::Duration;
use control_algorithms::controllers::void_control::VoidControl;
use serde::{Serialize, Deserialize};
use paho_mqtt as mqtt;
use paho_mqtt::Message;
use control_algorithms::ControlSimulation;
use control_algorithms::plants::lorenz_attractor::LorenzAttractor;
use control_algorithms::plants::pendulum::Pendulum;
use control_algorithms::plants::van_der_pol_oscillator::VanDerPolOscillator;
use control_algorithms::systems::autonomous_regularizer::AutonomousRegularizer;
use linear_algebra::vec::vec3D::Vec3D;

const SIMULATION_TYPE: &'static str = "2D";

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    x: Vec3D,
    v: Vec3D,
}

fn main() {

    // Mqtt connection
    println!("Setting up mqtt connection...");
    let cli = mqtt::AsyncClient::new("tcp://localhost:1883").unwrap();
    let tok = cli.connect(mqtt::ConnectOptions::new());
    tok.wait().unwrap();
    println!("Mqtt Works correctly.");

    /*
    // System Definition
    let system = AutonomousRegularizer::new(
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
    */

    /*
    let system = AutonomousRegularizer::new(
        VoidControl::new(),
        LorenzAttractor::new(
            10.0,
            2.666667,
            28.0,
            Vec3D::new(1.0, 1.0, 1.0),
            Vec3D::new(0.0, 0.0, 0.0),
        )
    );
     */

    let system = AutonomousRegularizer::new(
        VoidControl::new(),
        Pendulum::new(
            1.0,
            0.8,
            0.0,
            Vec3D::new(0.785, 0.000, 0.000),
            Vec3D::new(0.785, 0.000, 0.000),
        ),
    );

    /*
    let system = AutonomousRegularizer::new(
        VoidControl::new(),
        VanDerPolOscillator::new(
            4.0,
            Vec3D::new(0.785, 0.234, 0.0),
            Vec3D::new(0.0, 0.0, 0.0),
        )
    );
     */

    let mut sim = ControlSimulation::new(system, 30.0, 0.0001);

    loop {
        let (x, v) = sim.step();

        let data = Data { x, v };
        let data_json = serde_json::to_string(&data).unwrap();

        match SIMULATION_TYPE {
            "2D" => {
                cli.publish(Message::new("CTRL/out2d", data_json, 2));
            },
            "3D" => {
                cli.publish(Message::new("CTRL/out", data_json, 2));
            },
            _ => {
                println!("Something went wrong. Invalid simulation type selected")
            }
        }

        if sim.ended() { break; }
    }

    sleep(Duration::from_secs(3));

    match SIMULATION_TYPE {
        "2D" => {
            cli.publish(Message::new("CTRL/end2d", "1", 2));
        },
        "3D" => {
            cli.publish(Message::new("CTRL/end", "1", 2));
        },
        _ => {
            println!("Something went wrong. Invalid simulation type selected")
        }
    }

    println!("Simulation End.");
    sleep(Duration::from_secs(3));
    println!("Stop")
}

fn clear_screen() {
    if cfg!(unix) {
        std::process::Command::new("clear").status().unwrap();
    } else if cfg!(windows) {
        std::process::Command::new("cmd").args(&["/c", "cls"]).status().unwrap();
    }
}
