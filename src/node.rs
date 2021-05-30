use std::cell::RefCell;
use std::rc::Rc;

use super::component::*;
use super::terminal::Terminal;

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


    pub fn add_component(&self, c: Rc<dyn Component>, t: &Terminal) {
        self.children.borrow_mut().push(c);
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

    /// Test adding components
    #[test]
    fn node_add_component() {
        let n = Node::new("node");

        let cb: Rc<dyn Component> = Rc::new(CircuitBreaker::new("cb"));
        let ds: Rc<dyn Component> = Rc::new(Disconnector::new("ds"));
        let es: Rc<dyn Component> = Rc::new(EarthingSwitch::new("es"));   
        let vt: Rc<dyn Component> = Rc::new(VoltageTransformer::new("vt"));
        let tf: Rc<dyn Component> = Rc::new(Transformer::new("tf"));

        n.add_component(cb.clone(), cb.terminal(0).unwrap());
        n.add_component(ds.clone(), ds.terminal(0).unwrap());
        n.add_component(es.clone(), es.terminal(0).unwrap());
        n.add_component(vt.clone(), vt.terminal(0).unwrap());
        n.add_component(tf.clone(), tf.terminal(0).unwrap());

        let c = n.children.borrow();
        assert!(Rc::ptr_eq(&cb, &c[0]));
        assert!(Rc::ptr_eq(&ds, &c[1]));
        assert!(Rc::ptr_eq(&es, &c[2]));
        assert!(Rc::ptr_eq(&vt, &c[3]));
        assert!(Rc::ptr_eq(&tf, &c[4]));
    }
}
