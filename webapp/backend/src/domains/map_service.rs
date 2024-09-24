use crate::{
    errors::AppError,
    models::graph::{Edge, Graph, Node},
    models::graph_cache::GraphCache,
};
use std::sync::{Arc, RwLock};

pub trait MapRepository {
    async fn get_all_nodes(&self, area_id: Option<i32>) -> Result<Vec<Node>, sqlx::Error>;
    async fn get_all_edges(&self, area_id: Option<i32>) -> Result<Vec<Edge>, sqlx::Error>;
    async fn get_area_id_by_node_id(&self, node_id: i32) -> Result<i32, sqlx::Error>;
    async fn update_edge(
        &self,
        node_a_id: i32,
        node_b_id: i32,
        weight: i32,
    ) -> Result<(), sqlx::Error>;
}

#[derive(Debug)]
pub struct MapService<T: MapRepository + std::fmt::Debug> {
    repository: T,
}

impl<T: MapRepository + std::fmt::Debug> MapService<T> {
    pub fn new(repository: T) -> Self {
        MapService { repository }
    }

    pub async fn get_or_create_graph(
        &self,
        area_id: i32,
        cache: &GraphCache,
    ) -> Arc<RwLock<Graph>> {
        if let Some(graph) = cache.get_graph(area_id) {
            return graph;
        }

        let nodes = self.repository.get_all_nodes(Some(area_id)).await.unwrap();
        let edges = self.repository.get_all_edges(Some(area_id)).await.unwrap();

        let mut graph = Graph::new();
        for node in nodes {
            graph.add_node(node);
        }
        for edge in edges {
            graph.add_edge(edge);
        }

        cache.store_graph(area_id, graph);
        cache.get_graph(area_id).unwrap() // 新しく作成したグラフを返す
    }

    // node_a_idからarea_idを取得するためのメソッド
    pub async fn get_area_id_by_node_id(&self, node_id: i32) -> Result<i32, AppError> {
        self.repository
            .get_area_id_by_node_id(node_id)
            .await
            .map_err(|err| {
                log::error!("Failed to get area_id for node_id {}: {:?}", node_id, err);
                AppError::InternalServerError // 修正: 関数呼び出しではなくエラー型を直接返す
            })
    }

    // 既存のエッジ更新メソッド
    pub async fn update_edge(
        &self,
        node_a_id: i32,
        node_b_id: i32,
        weight: i32,
    ) -> Result<(), AppError> {
        self.repository
            .update_edge(node_a_id, node_b_id, weight)
            .await
            .map_err(|err| {
                log::error!(
                    "Failed to update edge from node {} to node {}: {:?}",
                    node_a_id,
                    node_b_id,
                    err
                );
                AppError::InternalServerError // 修正: 関数呼び出しではなくエラー型を直接返す
            })
    }
    // pub async fn update_edge(
    //     &self,
    //     node_a_id: i32,
    //     node_b_id: i32,
    //     weight: i32,
    // ) -> Result<(), AppError> {
    //     self.repository
    //         .update_edge(node_a_id, node_b_id, weight)
    //         .await?;

    //     Ok(())
    // }
}
