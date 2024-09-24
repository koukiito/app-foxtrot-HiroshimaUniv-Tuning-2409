use crate::models::graph::Graph;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct GraphCache {
    cache: RwLock<HashMap<i32, Arc<RwLock<Graph>>>>, // エリアIDに対するキャッシュ
}

impl GraphCache {
    pub fn new() -> Self {
        GraphCache {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_graph(&self, area_id: i32) -> Option<Arc<RwLock<Graph>>> {
        let cache = self.cache.read().unwrap();
        cache.get(&area_id).cloned()
    }

    pub fn store_graph(&self, area_id: i32, graph: Graph) {
        let mut cache = self.cache.write().unwrap();
        cache.insert(area_id, Arc::new(RwLock::new(graph)));
    }

    pub fn update_edge(&self, area_id: i32, node_a_id: i32, node_b_id: i32, weight: i32) {
        let cache = self.cache.read().unwrap();
        if let Some(graph) = cache.get(&area_id) {
            let mut graph = graph.write().unwrap(); // ミュータブルに借用
            graph.update_edge(node_a_id, node_b_id, weight); // グラフ内の辺を更新
        }
    }
}
