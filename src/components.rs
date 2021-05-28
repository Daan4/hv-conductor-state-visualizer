#[derive(Debug)]
pub enum ComponentType {
    Switch,
    EarthingSwitch,
    Measurement,
    Transformer
}

// enum SwitchPosition {
//     Open,
//     Close,
//     Invalid,
// }

pub trait Component {
    fn component_type(&self) -> ComponentType;
}

// have trait type implemented for all <T> where it is a component?

pub struct CircuitBreaker {
    //position: SwitchPosition,
}

pub struct Disconnector {
    //position: SwitchPosition,
}

pub struct EarthingSwitch {
    //position: SwitchPosition,
}

impl Component for CircuitBreaker {
    fn component_type(&self) -> ComponentType {
        ComponentType::Switch
    }
}

impl Component for Disconnector {
    fn component_type(&self) -> ComponentType {
        ComponentType::Switch
    }
}

impl Component for EarthingSwitch {
    fn component_type(&self) -> ComponentType {
        ComponentType::EarthingSwitch
    }
}
