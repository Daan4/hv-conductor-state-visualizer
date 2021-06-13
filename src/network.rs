use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::component::*;
use super::node::*;

/// A network which contains nodes and components
pub struct Network {
    name: String,
    nodes: RefCell<Vec<Rc<Node>>>,
    components: RefCell<Vec<Rc<dyn Component>>>,
}

impl Network {
    /// Constructor, sets the network name
    pub fn new(name: &str) -> Network {
        Network {
            name: name.to_string(),
            nodes: RefCell::new(vec![]),
            components: RefCell::new(vec![]),
        }
    }

    /// Return the network name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Check if a given name already exists in the network. Used to enforce unique names between all nodes and components within the network.
    fn check_name(&self, name: &str) -> Result<(), ()> {
        let node_index = self.nodes.borrow().iter().position(|x| x.name() == name);
        let component_index = self
            .components
            .borrow()
            .iter()
            .position(|x| x.name() == name);
        match (node_index, component_index) {
            (Some(_), Some(_)) => Err(()),
            (Some(_), None) => Err(()),
            (None, Some(_)) => Err(()),
            (None, None) => Ok(()),
        }
    }

    /// Create a node with the given name if the name is not already used in this network
    pub fn create_node(&self, name: &str) -> Result<(), String> {
        match self.check_name(name) {
            Err(_) => Err(format!("Failed to create node {} - A node or component with this name already exists in network {}", name, self.name())),
            Ok(()) => {
                let n = Rc::new(Node::new(name));
                self.nodes.borrow_mut().push(n);
                Ok(())
            }
        }
    }

    /// Remove a node with the given name from the network, if it exists
    pub fn remove_node(&self, name: &str) -> Result<(), String> {
        let index = self.nodes.borrow().iter().position(|x| x.name() == name);
        match index {
            Some(i) => {
                self.nodes.borrow_mut().remove(i);
                Ok(())
            }
            None => Err(format!(
                "Failed to remove node {} - No node with this name exists in network {}",
                name,
                self.name()
            )),
        }
    }

    /// Return the number of nodes in the network
    pub fn node_count(&self) -> usize {
        self.nodes.borrow().len()
    }

    /// Print each node in the network
    pub fn list_nodes(&self) {
        for node in self.nodes.borrow().iter() {
            println!("<{}", node);
        }
    }

    /// Get a reference to the node with the given name if it exists
    pub fn get_node(&self, name: &str) -> Result<Rc<Node>, String> {
        let nodes = self.nodes.borrow();
        let node = nodes.iter().find(|x| x.name() == name);
        match node {
            Some(node) => Ok(node.clone()),
            None => Err(format!(
                "Node with name {} does not exist in network {}",
                name,
                self.name()
            )),
        }
    }

    /// Create a component of a given [ComponentType] with a given name, if the name is not already in use in this network
    pub fn create_component<T: 'static + Component>(&self, name: &str) -> Result<(), String> {
        match self.check_name(name) {
            Err(_) => Err(format!("Failed to create component {} - A node or component with this name already exists in network {}", name, self.name())),
            Ok(()) => {
                let c = Rc::new(T::new(name));
                self.components.borrow_mut().push(c);
                Ok(())
            }
        }
    }

    /// Remove a component with the given name, if it exists in the network
    pub fn remove_component(&self, name: &str) -> Result<(), String> {
        let index = self
            .components
            .borrow()
            .iter()
            .position(|x| x.name() == name);
        match index {
            Some(i) => {
                self.components.borrow_mut().remove(i);
                Ok(())
            }
            None => Err(format!(
                "Failed to remove component {} - No component with this name exists in network {}",
                name,
                self.name()
            )),
        }
    }

    /// Return the number of components in the network
    pub fn component_count(&self) -> usize {
        self.components.borrow().len()
    }

    /// Print each component in the network
    pub fn list_components(&self) {
        for component in self.components.borrow().iter() {
            println!("<{}", component);
        }
    }

    /// Get a reference to the component with the given name, if it exists in the network
    pub fn get_component(&self, name: &str) -> Result<Rc<dyn Component>, String> {
        let components = self.components.borrow();
        let component = components.iter().find(|x| x.name() == name);
        match component {
            Some(component) => Ok(component.clone()),
            None => Err(format!(
                "Component with name {} does not exist in network {}",
                name,
                self.name()
            )),
        }
    }

    /// Connect a component terminal to a node. Returns an error if the component or node do not exist, or if the connection fails see [Component::connect]
    pub fn connect(
        &self,
        node_name: &str,
        component_name: &str,
        terminal: usize,
    ) -> Result<(), String> {
        let n = self.get_node(node_name);
        let c = self.get_component(component_name);
        match (n, c) {
            (Err(_), Err(_)) => Err(format!(
                "No node with name {} and component with name {} exist in network {}",
                node_name,
                component_name,
                self.name()
            )),
            (Err(_), _) => Err(format!(
                "No node with name {} exists in network {}",
                node_name,
                self.name()
            )),
            (_, Err(_)) => Err(format!(
                "No component with name {} exists in network {}",
                component_name,
                self.name()
            )),
            (Ok(n), Ok(c)) => c.connect(n, terminal),
        }
    }

    /// Disconnect a component from a node if it is connected. Returns an error if the component or node do not exist
    pub fn disconnect(&self, node_name: &str, component_name: &str) -> Result<(), String> {
        let n = self.get_node(node_name);
        let c = self.get_component(component_name);
        match (n, c) {
            (Err(_), Err(_)) => Err(format!(
                "No node with name {} and component with name {} exist in network {}",
                node_name,
                component_name,
                self.name()
            )),
            (Err(_), _) => Err(format!(
                "No node with name {} exists in network {}",
                node_name,
                self.name()
            )),
            (_, Err(_)) => Err(format!(
                "No component with name {} exists in network {}",
                component_name,
                self.name()
            )),
            (Ok(n), Ok(c)) => c.disconnect(n),
        }
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
        let net = Network::new("net");
        net.create_node("node").unwrap();
        net.create_component::<CircuitBreaker>("cb").unwrap();

        assert!(net.connect("does not", "exist", 0).is_err());
        assert!(net.disconnect("node", "cb").is_err());
        assert!(net.connect("node", "cb", 2).is_err());
        assert!(net.connect("node", "cb", 0).is_ok());
        assert!(net.connect("node", "cb", 1).is_err());
        assert!(net.disconnect("node", "cb").is_ok());
        assert!(net.connect("node", "cb", 1).is_ok());
    }
}
