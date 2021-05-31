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

    pub fn create_node(&self, name: &'static str) -> Result<(), String> {
        let index = self.nodes.borrow().iter().position(|x| x.name() == name);
        match index {
            Some(_) => Err(format!("Failed to create node {} - A node with this name already exists in network {}", name, self.name())),
            None => {
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

    }

    pub fn get_node(&self, name: &str) {
    }

    pub fn create_component<T: 'static + Component>(&self, name: &'static str) -> Result<(), String> {
        let index = self.components.borrow().iter().position(|x| x.name() == name);
        match index {
            Some(_) => Err(format!("Failed to create node {} - A node with this name already exists in network {}", name, self.name())),
            None => {
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
            None => Err(format!("Failed to remove component {} - No component with this name exists in this network", name))
        }
    }

    pub fn component_count(&self) ->usize {
        self.components.borrow().len()
    }

    pub fn list_components() {

    }

    pub fn get_component() {

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
        net.create_component::<CircuitBreaker>("cb").unwrap();
        assert!(net.create_component::<Disconnector>("cb").is_err());
        assert_eq!(net.component_count(), 1);

        net.create_component::<Disconnector>("ds").unwrap();
        assert_eq!(net.component_count(), 2);

        net.remove_component("cb").unwrap();
        assert_eq!(net.component_count(), 1);
        assert!(net.remove_component("cb").is_err());

        net.remove_component("ds").unwrap();
        assert_eq!(net.component_count(), 0);
    }

    #[test]
    fn network_node() {
        let net = Network::new("net");

        assert_eq!(net.node_count(), 0);
        net.create_node("node").unwrap();
        assert!(net.create_node("node").is_err());
        assert_eq!(net.node_count(), 1);

        net.create_node("node2").unwrap();
        assert_eq!(net.node_count(), 2);

        net.remove_node("node").unwrap();
        assert!(net.remove_node("node").is_err());
        assert_eq!(net.node_count(), 1);

        net.remove_node("node2").unwrap();
        assert_eq!(net.node_count(), 0);
    }
}
