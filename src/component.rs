use super::position::SwitchgearPosition;
use super::terminal::Terminal;

/// Component
pub trait Component {
    fn new(name: Option<&'static str>) -> Self where Self: Sized;

    fn r#type(&self) -> ComponentType;
    fn name(&self) -> &'static str;
    fn terminal(&self, index: usize) -> Result<&Terminal, String>;   
}

/// Component Type
#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Switch,
    EarthingSwitch,
    Measurement,
    Transformer,
}

/// Circuit Breaker
pub struct CircuitBreaker {
    name: &'static str,
    pub position: SwitchgearPosition,
    terminals: [Terminal; 2],
}

impl Component for CircuitBreaker {
    fn new(name: Option<&'static str>) -> CircuitBreaker {
        CircuitBreaker { 
            name: match name {
            Some(name) => name,
            None => "cb"
        },  
            position: SwitchgearPosition::new(), 
            terminals: [Terminal::new(), Terminal::new()],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Switch
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&Terminal, String> {
        Ok(&self.terminals[index])
    }
}

/// Disconnector
pub struct Disconnector {
    name: &'static str,
    pub position: SwitchgearPosition,
    terminals: [Terminal; 2],
}

impl Component for Disconnector {
    fn new(name: Option<&'static str>) -> Disconnector {
        Disconnector { 
            name: match name {
            Some(name) => name,
            None => "ds"
        },  
            position: SwitchgearPosition::new(), 
            terminals: [Terminal::new(), Terminal::new()],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Switch
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&Terminal, String> {
        Ok(&self.terminals[index])
    }
}

/// Earthing Switch
pub struct EarthingSwitch {
    name: &'static str,
    pub position: SwitchgearPosition,
    terminals: [Terminal; 1],
}

impl Component for EarthingSwitch {
    fn new(name: Option<&'static str>) -> EarthingSwitch {
        EarthingSwitch { 
            name: match name {
                Some(name) => name,
                None => "es"
            },
            position: SwitchgearPosition::new(), 
            terminals: [Terminal::new(); 1],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::EarthingSwitch
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&Terminal, String> {
        Ok(&self.terminals[index])
    }
}

/// Voltage Transformer
pub struct VoltageTransformer {
    name: &'static str,
    terminals: [Terminal; 1],
}

impl Component for VoltageTransformer {
    fn new(name: Option<&'static str>) -> VoltageTransformer {
        VoltageTransformer { 
            name: match name {
                Some(name) => name,
                None => "vt"
            },
            terminals: [Terminal::new(); 1],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Measurement
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&Terminal, String> {
        Ok(&self.terminals[index])
    }
}

/// Transformer
pub struct Transformer {
    name: &'static str,
    terminals:[Terminal; 3],
}

impl Component for Transformer {
    fn new(name: Option<&'static str>) -> Transformer {
        Transformer { 
            name: match name {
                Some(name) => name,
                None => "tf"
            },
            terminals: [Terminal::new(), Terminal::new(), Terminal::new()],
        }
    }

    fn r#type(&self) -> ComponentType {
        ComponentType::Transformer
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn terminal(&self, index: usize) -> Result<&Terminal, String> {
        Ok(&self.terminals[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn component_types() {
        assert_eq!(CircuitBreaker::new(None).r#type(), ComponentType::Switch);
        assert_eq!(Disconnector::new(None).r#type(), ComponentType::Switch);
        assert_eq!(EarthingSwitch::new(None).r#type(), ComponentType::EarthingSwitch);
        assert_eq!(VoltageTransformer::new(None).r#type(), ComponentType::Measurement);
        assert_eq!(Transformer::new(None).r#type(), ComponentType::Transformer)
    }

    #[test]
    fn component_terminals() {
        assert_eq!(CircuitBreaker::new(None).terminals.len(), 2);
        assert_eq!(Disconnector::new(None).terminals.len(), 2);
        assert_eq!(EarthingSwitch::new(None).terminals.len(), 1);
        assert_eq!(VoltageTransformer::new(None).terminals.len(), 1);
        assert_eq!(Transformer::new(None).terminals.len(), 3);
    }
}
