use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

use super::component::*;

pub struct Node {
    name: &'static str,
    children: RefCell<Vec<Rc<dyn Component>>>,
}

impl Node {
    pub fn new(name: &'static str) -> Node {
        Node {
            name,
            children: RefCell::new(vec![]),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn add_component(&self, c: Rc<dyn Component>) -> Result<(), String> {
        let index = self.children.borrow().iter().position(|x| Rc::ptr_eq(x, &c));
        match index {
            Some(_) => Err(format!("Failed to add component {} to node {} - Component already exists on node", c.name(), self.name())),
            None => {
                self.children.borrow_mut().push(c);
                Ok(())
            }
        }
    }

    pub fn remove_component(&self, c: Rc<dyn Component>) -> Result<(), String> {
        let index = self.children.borrow().iter().position(|x| Rc::ptr_eq(x, &c));
        match index {
            Some(i) => {
                self.children.borrow_mut().remove(i);
                Ok(())
            },
            None => Err(format!("Failed to remove component {} from node {} - Component does not exist on node", c.name(), self.name()))
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_name() {
        let n = Node::new("node");
        assert_eq!(n.name(), "node")
    }

    #[test]
    fn node_component() {
        let n = Node::new("node");

        let cb: Rc<dyn Component> = Rc::new(CircuitBreaker::new("cb"));
        let ds: Rc<dyn Component> = Rc::new(Disconnector::new("ds"));

        assert!(n.children.borrow().len() == 0);
        n.add_component(cb.clone()).unwrap();
        assert!(Rc::ptr_eq(&cb, &n.children.borrow()[0]));
        assert!(n.add_component(cb.clone()).is_err());
        assert!(n.children.borrow().len() == 1);

        n.add_component(ds.clone()).unwrap();
        assert!(Rc::ptr_eq(&ds, &n.children.borrow()[1]));
        assert!(n.children.borrow().len() == 2);

        n.remove_component(cb.clone()).unwrap();
        assert!(n.remove_component(cb).is_err());
        assert!(Rc::ptr_eq(&ds, &n.children.borrow()[0]));
        assert!(n.children.borrow().len() == 1);
        n.remove_component(ds).unwrap();
        assert!(n.children.borrow().len() == 0);
    }
}
