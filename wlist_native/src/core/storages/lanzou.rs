use std::sync::Arc;

use dashmap::DashMap;

use crate::core::storages::Node;

pub struct LanzouStorage {
    pub map: Arc<DashMap<i64, Node>>,
    pub root: Node,
}

impl LanzouStorage {
    pub fn new(root: i64) -> Self {
        let map = Arc::new(DashMap::new());
        let root = Node::root(&map, root);
        Self { map, root, }
    }
}
