use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct EnqueueInteractionRequest {
    pub interaction_id: Uuid,
    pub queue_id: Uuid,
    pub priority: i32,
}
