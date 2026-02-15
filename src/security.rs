//! Support for security policies

use xrust::item::Node;
use xrust::transform::Transform;

pub struct Policy<N: Node> {
    name: String,
    features: Vec<Feature<N>>,
}

impl<N: Node> Policy<N> {
    pub fn new(name: String, features: Vec<Feature<N>>) -> Self {
        Self { name, features }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    // should this be Vec<&Feature>?
    pub fn features(&self) -> &Vec<Feature<N>> {
        &self.features
    }
}

pub enum Feature<N: Node> {
    Value(String),
    Template(Transform<N>),
}
