use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use super::position::SwitchgearPosition;
use super::terminal::Terminal;
use super::node::Node;

/// Component
pub trait Component {
    fn new(name: &str) -> Self where Self: Sized;

    fn r#type(&self) -> ComponentType;
    fn name(&self) -> &String;
    fn terminal(&self, index: usize) -> Result<&RefCell<Terminal>, String>; 

    /// Only allow a connection if no other terminal is connected to the given node already
    fn connect(&self, node: Rc<Node>, index: usize) -> Result<(), String> {
        let mut i = 0;
        loop {
            if i == index {
                i += 1;
                continue;
            }
            match self.terminal(i) {
                Err(_) => { 
                    break; 
                },
                Ok(t) => {
                    if let Ok(n) = t.borrow().get_node() {
                        if Rc::ptr_eq(&node, &n) {
                            return Err(format!("Component {} is already connected to node {} on terminal {}", self.name(), node.name(), i));
                        }
                    }
                },
            }
            i += 1;
        }
        let t = self.terminal(index)?;
        t.borrow_mut().connect(node)?;
        Ok(())
    }

    fn disconnect(&self, node: Rc<Node>) -> Result<(), String> {
        let mut i = 0;
        while let Ok(t) = self.terminal(i) {
            let mut t = t.borrow_mut();
            if let Ok(n) = t.get_node() {
                if Rc::ptr_eq(&n, &node) {
                    return t.disconnect();
                }
            }
            i += 1;
        }
        Err(format!("Component {} is not connnected to node {}", self.name(), node.name()))
    }
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
    name: String,
    pub position: SwitchgearPosition,
    terminals: [RefCell<Terminal>; 2],
}

impl Component for CircuitBreaker {
    fn new(name: &str) -> CircuitBreaker {
        CircuitBreaker { 
            name: name.to_string(),
            position: SwitchgearPosition::new(), 
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::CircuitBreaker
    }

    fn name(&self) -> &String {
        &self.name
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
    name: String,
    pub position: SwitchgearPosition,
    terminals: [RefCell<Terminal>; 2],
}

impl Component for Disconnector {
    fn new(name: &str) -> Disconnector {
        Disconnector { 
            name: name.to_string(),
            position: SwitchgearPosition::new(), 
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Disconnector
    }

    fn name(&self) -> &String {
        &self.name
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
    name: String,
    pub position: SwitchgearPosition,
    terminals: [RefCell<Terminal>; 1],
}

impl Component for EarthingSwitch {
    fn new(name: &str) -> EarthingSwitch {
        EarthingSwitch { 
            name: name.to_string(),
            position: SwitchgearPosition::new(), 
            terminals: [RefCell::new(Terminal::new()); 1],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::EarthingSwitch
    }

    fn name(&self) -> &String {
        &self.name
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
    name: String,
    terminals: [RefCell<Terminal>; 1],
}

impl Component for VoltageTransformer {
    fn new(name: &str) -> VoltageTransformer {
        VoltageTransformer { 
            name: name.to_string(),
            terminals: [RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Measurement
    }

    fn name(&self) -> &String {
        &self.name
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
    name: String,
    terminals: [RefCell<Terminal>; 3],
}

impl Component for Transformer {
    fn new(name: &str) -> Transformer {
        Transformer { 
            name: name.to_string(),
            terminals: [RefCell::new(Terminal::new()), RefCell::new(Terminal::new()), RefCell::new(Terminal::new())],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Transformer
    }

    fn name(&self) -> &String {
        &self.name
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

    #[test]
    fn component_connect() {
        let n = Rc::new(Node::new("node"));
        let n2 = Rc::new(Node::new("node2"));
        let cb = CircuitBreaker::new("cb");

        assert!(cb.disconnect(n.clone()).is_err());
        assert!(cb.connect(n.clone(), 2).is_err());
        assert!(cb.connect(n.clone(), 0).is_ok());
        assert!(cb.connect(n.clone(), 0).is_err());
        assert!(cb.connect(n.clone(), 1).is_err());
        assert!(cb.connect(n2.clone(), 1).is_ok());
        assert!(cb.disconnect(n2.clone()).is_ok());
        assert!(cb.connect(n2.clone(), 0).is_err());
    }
}
