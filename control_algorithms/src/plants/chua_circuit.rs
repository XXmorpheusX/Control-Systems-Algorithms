
pub mod chua_circuit {
    use crate::plants::Plant;

    pub struct ChuaCircuit {

    }

    impl ChuaCircuit {
        pub fn new() -> Box<dyn Plant> {
            Box::new( ChuaCircuit {} )
        }
    }

    impl Plant for ChuaCircuit {

    }
}
