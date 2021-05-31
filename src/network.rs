use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use super::component::*;
use super::node::*;

pub struct Network {
    name: &'static str,
    nodes: RefCell<Vec<Rc<Node>>>,
    components: RefCell<Vec<Rc<dyn Component>>>,
}

impl Network {
    pub fn new(name: &'static str) -> Network {
        Network {
            name,
            nodes: RefCell::new(vec![]),
            components: RefCell::new(vec![]),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Each node and component in a network must have a uniquely identifying name
    fn check_name(&self, name: &str) -> Result<(), ()> {
        let node_index = self.nodes.borrow().iter().position(|x| x.name() == name);
        let component_index = self.components.borrow().iter().position(|x| x.name() == name);
        match (node_index, component_index) {
            (Some(_), Some(_)) => Err(()),
            (Some(_), None) => Err(()),
            (None, Some(_)) => Err(()),
            (None, None) => Ok(())
        }
    }

    pub fn create_node(&self, name: &'static str) -> Result<(), String> {
        match self.check_name(name) {
            Err(_) => Err(format!("Failed to create node {} - A node or component with this name already exists in network {}", name, self.name())),
            Ok(()) => {
                let n = Rc::new(Node::new(name));
                self.nodes.borrow_mut().push(n);
                Ok(())
            }
        }
    }

    pub fn remove_node(&self, name: &str) -> Result<(), String> {
        let index = self.nodes.borrow().iter().position(|x| x.name() == name);
        match index {
            Some(i) => {
                self.nodes.borrow_mut().remove(i);
                Ok(())
            },
            None => Err(format!("Failed to remove node {} - No node with this name exists in network {}", name, self.name()))
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.borrow().len()
    }

    pub fn list_nodes(&self) {
        for node in self.nodes.borrow().iter() {
            println!("{}", node);
        }
    }

    pub fn get_node(&self, name: &str) -> Result<Rc<Node>, String> {
        let nodes = self.nodes.borrow();
        let node = nodes.iter().find(|x| x.name() == name);
        match node {
            Some(node) => Ok(node.clone()),
            None => Err(format!("Node with name {} does not exist in network {}", name, self.name()))
        }     
    }

    pub fn create_component<T: 'static + Component>(&self, name: &'static str) -> Result<(), String> {
        match self.check_name(name) {
            Err(_) => Err(format!("Failed to create component {} - A node or component with this name already exists in network {}", name, self.name())),
            Ok(()) => {
                let c = Rc::new(T::new(name));
                self.components.borrow_mut().push(c);
                Ok(())
            }
        }
    }

    pub fn remove_component(&self, name: &str) -> Result<(), String> {
        let index = self.components.borrow().iter().position(|x| x.name() == name);
        match index {
            Some(i) => {
                self.components.borrow_mut().remove(i);
                Ok(())
            },
            None => Err(format!("Failed to remove component {} - No component with this name exists in network {}", name, self.name()))
        }
    }

    pub fn component_count(&self) -> usize {
        self.components.borrow().len()
    }

    pub fn list_components(&self) {
        for component in self.components.borrow().iter() {
            println!("{}", component);
        }
    }

    pub fn get_component(&self, name: &str) -> Result<Rc<dyn Component>, String> {
        let components = self.components.borrow();
        let component = components.iter().find(|x| x.name() == name);
        match component {
            Some(component) => Ok(component.clone()),
            None => Err(format!("Component with name {} does not exist in network {}", name, self.name()))
        }        
    }

    pub fn connect() -> Result<(), String> {
        Ok(())
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Network {}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_name() {
        let net = Network::new("net");
        assert_eq!(net.name(), "net")
    }

    #[test]
    fn network_component() {
        let net = Network::new("net");

        assert_eq!(net.component_count(), 0);
        assert!(net.get_component("cb").is_err());
        net.create_component::<CircuitBreaker>("cb").unwrap();
        assert!(net.get_component("cb").unwrap().name() == "cb");
        assert!(net.create_node("cb").is_err());
        assert!(net.create_component::<Disconnector>("cb").is_err());
        assert_eq!(net.component_count(), 1);

        net.create_component::<Disconnector>("ds").unwrap();
        assert!(net.get_component("ds").unwrap().name() == "ds");
        assert_eq!(net.component_count(), 2);

        net.remove_component("cb").unwrap();
        assert!(net.get_component("cb").is_err());
        assert_eq!(net.component_count(), 1);
        assert!(net.remove_component("cb").is_err());

        net.remove_component("ds").unwrap();
        assert_eq!(net.component_count(), 0);
    }

    #[test]
    fn network_node() {
        let net = Network::new("net");

        assert_eq!(net.node_count(), 0);
        assert!(net.get_node("node").is_err());
        net.create_node("node").unwrap();
        assert!(net.get_node("node").unwrap().name() == "node");
        assert!(net.create_component::<CircuitBreaker>("node").is_err());
        assert!(net.create_node("node").is_err());
        assert_eq!(net.node_count(), 1);        

        net.create_node("node2").unwrap();
        assert!(net.get_node("node2").unwrap().name() == "node2");
        assert_eq!(net.node_count(), 2);

        net.remove_node("node").unwrap();
        assert!(net.get_node("node").is_err());
        assert!(net.remove_node("node").is_err());
        assert_eq!(net.node_count(), 1);

        net.remove_node("node2").unwrap();
        assert_eq!(net.node_count(), 0);
    }

    #[test]
    fn network_connect() {

    }
}
