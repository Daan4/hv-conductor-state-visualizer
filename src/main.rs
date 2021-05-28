use hvcv::components::{CircuitBreaker, Disconnector, EarthingSwitch, Component};

fn main() {
    let cb = CircuitBreaker {};
    let dis = Disconnector {};
    let es = EarthingSwitch {};
    println!("{:?} {:?} {:?}", Component::component_type(&cb), Component::component_type(&dis), Component::component_type(&es));
}
