use crate::{
    domains::{dto::map::UpdateEdgeRequestDto, map_service::MapService},
    errors::AppError,
    models::graph_cache::GraphCache, // 追加
    repositories::map_repository::MapRepositoryImpl,
};
use actix_web::{web, HttpResponse};
pub async fn update_edge_handler(
    service: web::Data<MapService<MapRepositoryImpl>>,
    req: web::Json<UpdateEdgeRequestDto>,
    cache: web::Data<GraphCache>,
) -> Result<HttpResponse, AppError> {
    log::info!(
        "Received request to update edge: node_a_id: {}, node_b_id: {}, weight: {}",
        req.node_a_id,
        req.node_b_id,
        req.weight
    );

    // node_a_idからarea_idを特定
    let area_id = match service.get_area_id_by_node_id(req.node_a_id).await {
        Ok(id) => id,
        Err(err) => {
            log::error!(
                "Failed to get area_id for node_a_id {}: {:?}",
                req.node_a_id,
                err
            );
            return Err(AppError::InternalServerError); // 修正
        }
    };

    match service
        .update_edge(req.node_a_id, req.node_b_id, req.weight)
        .await
    {
        Ok(_) => {
            log::info!("Edge updated successfully in DB, updating cache...");
            cache.update_edge(area_id, req.node_a_id, req.node_b_id, req.weight); // キャッシュを更新
            log::info!("Cache updated successfully for area_id: {}", area_id);
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => {
            log::error!("Error updating edge: {:?}", err);
            Err(AppError::InternalServerError) // 修正
        }
    }
}
