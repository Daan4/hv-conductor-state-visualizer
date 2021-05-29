use super::component::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    children: RefCell<Vec<Rc<dyn Component>>>,
}

impl Node {
    fn new() -> Node {
        Node {
            children: RefCell::new(vec![]),
        }
    }

    fn add_component(&self, c: Rc<dyn Component>) {
        self.children.borrow_mut().push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test adding components
    #[test]
    fn node_add_component() {
        let n = Node::new();
        let cb: Rc<dyn Component> = Rc::new(CircuitBreaker::new());
        let ds: Rc<dyn Component> = Rc::new(Disconnector::new());
        let es: Rc<dyn Component> = Rc::new(EarthingSwitch::new());   
        let vt: Rc<dyn Component> = Rc::new(VoltageTransformer::new());
        let tf: Rc<dyn Component> = Rc::new(Transformer::new());

        n.add_component(cb.clone());
        n.add_component(ds.clone());
        n.add_component(es.clone());
        n.add_component(vt.clone());
        n.add_component(tf.clone());

        let c = n.children.borrow();
        assert!(Rc::ptr_eq(&cb, &c[0]));
        assert!(Rc::ptr_eq(&ds, &c[1]));
        assert!(Rc::ptr_eq(&es, &c[2]));
        assert!(Rc::ptr_eq(&vt, &c[3]));
        assert!(Rc::ptr_eq(&tf, &c[4]));
    }
}
