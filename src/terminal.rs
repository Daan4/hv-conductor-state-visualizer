use std::rc::Rc;

use super::node::Node;

pub struct Terminal {
    node: Option<Rc<Node>>,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            node: None,
        }
    }

    pub fn connect(&mut self, node: Rc<Node>) -> Result<(), &str> {
        match self.node {
            Some(_) => Err("Terminal already connected"),
            None => {
                self.node = Some(node.clone());
                Ok(())
            },
        }
    }

    pub fn disconnect(&mut self) -> Result<(), &str> {
        match self.node {
            Some(_) => {
                self.node = None;
                Ok(())
            },
            None => Err("Terminal not connected"),
        }
    }

    pub fn get_node(&self) -> Result<Rc<Node>, &str> {
        match self.node.clone() {
            Some(node) => Ok(node),
            None => Err("Terminal not connected"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_connect() {
        let n = Rc::new(Node::new(None));
        let mut t = Terminal::new();

        assert!(t.disconnect().is_err());
        assert!(t.get_node().is_err());

        t.connect(n.clone()).unwrap();
        assert!(Rc::ptr_eq(&t.get_node().unwrap(), &n));
        assert!(t.connect(n).is_err());

        t.disconnect().unwrap();
        assert!(t.get_node().is_err());
    }
}
