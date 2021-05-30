use std::rc::Weak;

use super::node::Node;

pub struct Terminal {
    node: Option<Weak<Node>>,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            node: None,
        }
    }

    pub fn connect() {

    }

    pub fn get_node() {

    }
}
