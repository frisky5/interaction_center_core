use crate::contants::InteractionState::Enqueued;
use crate::core::Core;
use crate::requests::EnqueueInteractionRequest;
use crate::responses::GenericResponse;
use actix_web::{put, web, HttpResponse};
use std::sync::RwLock;

#[put("/v1/queues/interactions")]
pub async fn enqueue_interaction(
    request: web::Json<EnqueueInteractionRequest>,
    core: web::Data<RwLock<Core>>,
) -> HttpResponse {
    let core_guard = core.read().unwrap();
    let interactions_guard = core_guard.interactions.read().unwrap();
    let queues_guard = core_guard.interactions.read().unwrap();
    let interaction_guard = interactions_guard
        .get(&request.interaction_id)
        .unwrap()
        .write()
        .unwrap();
    let queue_guard = queues_guard
        .get(&request.queue_id)
        .unwrap()
        .write()
        .unwrap();

    if !interaction_guard.is_state_allowed(Enqueued) {
        return HttpResponse::BadRequest().json(GenericResponse {
            error: true,
            message: String::from("interaction is already in queue"),
        });
    }

    HttpResponse::Ok().json(GenericResponse {
        error: false,
        message: String::from("success"),
    })
}
