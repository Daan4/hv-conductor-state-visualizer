use super::node::Node;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Switch,
    EarthingSwitch,
    Measurement,
    Transformer,
}

/// Switchgear Position 
#[derive(PartialEq)]
enum SwitchPosition {
    Open,
    Close,
}

struct SwitchgearPosition {
    position: SwitchPosition,
}

impl SwitchgearPosition {
    fn new() -> SwitchgearPosition {
        SwitchgearPosition {
            position: SwitchPosition::Open,
        }
    }

    fn is_closed(&self) -> bool {
        self.position == SwitchPosition::Close
    }

    fn is_open(&self) -> bool {
        self.position == SwitchPosition::Open
    }

    fn close(&mut self) {
        self.position = SwitchPosition::Close;
    }

    fn open(&mut self) {
        self.position = SwitchPosition::Open;
    }
}

/// Trait for component type
pub trait Component {
    fn component_type(&self) -> ComponentType;
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

impl Component for VoltageTransformer {
    fn component_type(&self) -> ComponentType {
        ComponentType::Measurement
    }
}

impl Component for Transformer {
    fn component_type(&self) -> ComponentType {
        ComponentType::Transformer
    }
}

struct Terminal {
    node: Option<Weak<Node>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            node: None,
        }
    }
}

pub struct CircuitBreaker {
    position: SwitchgearPosition,
    terminals: [Terminal; 2],
}

impl CircuitBreaker {
    pub fn new() -> CircuitBreaker {
        CircuitBreaker {
            position: SwitchgearPosition::new(),
            terminals: [Terminal::new(), Terminal::new()],
        }
    }
}

pub struct Disconnector {
    position: SwitchgearPosition,
    terminals: [Terminal; 2],
}

impl Disconnector {
    pub fn new() -> Disconnector {
        Disconnector {
            position: SwitchgearPosition::new(),
            terminals: [Terminal::new(), Terminal::new()],
        }
    }
}

pub struct EarthingSwitch {
    position: SwitchgearPosition,
    terminals: [Terminal; 1],
}

impl EarthingSwitch {
    pub fn new() -> EarthingSwitch {
        EarthingSwitch {
            position: SwitchgearPosition::new(),
            terminals: [Terminal::new()],
        }
    }
}

pub struct VoltageTransformer {
    terminals: [Terminal; 1],
}

impl VoltageTransformer {
    pub fn new() -> VoltageTransformer {
        VoltageTransformer {
            terminals: [Terminal::new()],
        }
    }
}

pub struct Transformer {
    terminals:[Terminal; 3],
}

impl Transformer {
    pub fn new() -> Transformer {
        Transformer {
            terminals: [Terminal::new(), Terminal::new(), Terminal::new()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn component_types() {
        assert_eq!(CircuitBreaker::new().component_type(), ComponentType::Switch);
        assert_eq!(Disconnector::new().component_type(), ComponentType::Switch);
        assert_eq!(EarthingSwitch::new().component_type(), ComponentType::EarthingSwitch);
        assert_eq!(VoltageTransformer::new().component_type(), ComponentType::Measurement);
        assert_eq!(Transformer::new().component_type(), ComponentType::Transformer)
    }

    #[test]
    fn component_terminals() {
        assert_eq!(CircuitBreaker::new().terminals.len(), 2);
        assert_eq!(Disconnector::new().terminals.len(), 2);
        assert_eq!(EarthingSwitch::new().terminals.len(), 1);
        assert_eq!(VoltageTransformer::new().terminals.len(), 1);
        assert_eq!(Transformer::new().terminals.len(), 3);
    }

    #[test]
    fn switchgear_position() {
        let mut cb = CircuitBreaker::new();
        let mut ds = Disconnector::new();
        let mut es = EarthingSwitch::new();

        assert!(cb.position.is_open());
        cb.position.close();
        assert!(cb.position.is_closed());
        cb.position.open();
        assert!(cb.position.is_open());

        assert!(ds.position.is_open());
        ds.position.close();
        assert!(ds.position.is_closed());
        ds.position.open();
        assert!(ds.position.is_open());

        assert!(es.position.is_open());
        es.position.close();
        assert!(es.position.is_closed());
        es.position.open();
        assert!(es.position.is_open());
    }
}
