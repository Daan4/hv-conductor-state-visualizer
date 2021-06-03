use std::fmt;
use std::cell::RefCell;

use super::position::SwitchgearPosition;
use super::terminal::Terminal;

/// Component
pub trait Component {
    fn new(name: &'static str) -> Self where Self: Sized;

    fn r#type(&self) -> ComponentType;
    fn name(&self) -> &'static str;
    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String>;   
}

impl fmt::Display for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Component {} of type {:?}", self.name(), self.r#type())
    }
}

/// Component Type
#[derive(Debug, PartialEq)]
pub enum ComponentType {
    CircuitBreaker,
    Disconnector,
    EarthingSwitch,
    Measurement,
    Transformer,
}

/// Circuit Breaker
pub struct CircuitBreaker {
    name: &'static str,
    pub position: SwitchgearPosition,
    terminals: [RefCell<Terminal>; 2],
}

impl Component for CircuitBreaker {
    fn new(name: &'static str) -> CircuitBreaker {
        CircuitBreaker { 
            name,
            position: SwitchgearPosition::new(), 
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::CircuitBreaker
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

/// Disconnector
pub struct Disconnector {
    name: &'static str,
    pub position: SwitchgearPosition,
    terminals: [RefCell<Terminal>; 2],
}

impl Component for Disconnector {
    fn new(name: &'static str) -> Disconnector {
        Disconnector { 
            name,
            position: SwitchgearPosition::new(), 
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Disconnector
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

/// Earthing Switch
pub struct EarthingSwitch {
    name: &'static str,
    pub position: SwitchgearPosition,
    terminals: [RefCell<Terminal>; 1],
}

impl Component for EarthingSwitch {
    fn new(name: &'static str) -> EarthingSwitch {
        EarthingSwitch { 
            name,
            position: SwitchgearPosition::new(), 
            terminals: [RefCell::new(Terminal::new()); 1],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::EarthingSwitch
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

/// Voltage Transformer
pub struct VoltageTransformer {
    name: &'static str,
    terminals: [RefCell<Terminal>; 1],
}

impl Component for VoltageTransformer {
    fn new(name: &'static str) -> VoltageTransformer {
        VoltageTransformer { 
            name,
            terminals: [RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Measurement
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

/// Transformer
pub struct Transformer {
    name: &'static str,
    terminals: [RefCell<Terminal>; 3],
}

impl Component for Transformer {
    fn new(name: &'static str) -> Transformer {
        Transformer { 
            name,
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Transformer
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String> {
        match self.terminals.get(index) {
            Some(t) => Ok(t),
            None => Err(format!("Component {} of type {:?} does not have a terminal with index {}; it only has {} terminals", self.name, self.r#type(), index, self.terminals.len())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_components() -> (CircuitBreaker, Disconnector, EarthingSwitch, VoltageTransformer, Transformer) {
        let cb = CircuitBreaker::new("cb");
        let ds = Disconnector::new("ds");
        let es = EarthingSwitch::new("es");
        let vt = VoltageTransformer::new("vt");
        let tf = Transformer::new("tf");

        (cb, ds, es, vt, tf)
    }

    #[test]
    fn component_names() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert_eq!(cb.name(), "cb");
        assert_eq!(ds.name(), "ds");
        assert_eq!(es.name(), "es");
        assert_eq!(vt.name(), "vt");
        assert_eq!(tf.name(), "tf");
    }
    
    #[test]
    fn component_types() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert_eq!(cb.r#type(), ComponentType::CircuitBreaker);
        assert_eq!(ds.r#type(), ComponentType::Disconnector);
        assert_eq!(es.r#type(), ComponentType::EarthingSwitch);
        assert_eq!(vt.r#type(), ComponentType::Measurement);
        assert_eq!(tf.r#type(), ComponentType::Transformer)
    }

    #[test]
    fn component_terminals() {
        let (cb, ds, es, vt, tf) = create_test_components();

        assert_eq!(cb.terminals.len(), 2);
        assert_eq!(ds.terminals.len(), 2);
        assert_eq!(es.terminals.len(), 1);
        assert_eq!(vt.terminals.len(), 1);
        assert_eq!(tf.terminals.len(), 3);

        assert!(cb.terminal(2).is_err());
        assert!(ds.terminal(2).is_err());
        assert!(es.terminal(1).is_err());
        assert!(vt.terminal(1).is_err());
        assert!(tf.terminal(3).is_err());
    }
}
