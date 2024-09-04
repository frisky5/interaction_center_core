use crate::core::Core;
use crate::interaction::Interaction;
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Deserialize)]
struct InteractionRequest {
    channel: i32,
}

#[derive(Serialize)]
struct InteractionResponse {
    id: String,
    created_at: Option<String>,
    channel: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[post("/v1/interactions")]
pub async fn create_interaction(
    request: web::Json<InteractionRequest>,
    core: web::Data<RwLock<Core>>,
    pool: web::Data<Pool>,
) -> HttpResponse {
    let client = pool.get().await.unwrap();
    let interaction_id = Uuid::now_v7();
    let created_at = Utc::now();
    client
        .execute(
            "INSERT INTO interactions (id, created_at, channel) VALUES ($1,$2,$3)",
            &[&interaction_id, &created_at, &request.channel],
        )
        .await
        .unwrap();
    let mut _core = core.read().unwrap();
    _core.interactions.write().unwrap().insert(
        interaction_id,
        Arc::from(RwLock::new(Interaction::new(interaction_id))),
    );

    HttpResponse::Ok().json(InteractionResponse {
        id: interaction_id.to_string(),
        created_at: Some(created_at.to_string()),
        channel: Some(request.channel),
        message: None,
    })
}
