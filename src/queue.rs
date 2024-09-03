use crate::agent::Agent;
use crate::core::Core;
use crate::interaction::Interaction;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug)]
pub struct Queues {
    queues: RwLock<HashMap<Uuid, RwLock<Queue>>>,
}

#[derive(Debug)]
pub struct Queue {
    pub id: Uuid,
    pub algorithm: i32,
    pub channel: i32,
    pub interactions: RwLock<Vec<Arc<Interaction>>>,
    pub agents: RwLock<Vec<Arc<Agent>>>,
}
impl Queue {
    pub fn new(id: Uuid, algorithm: i32, channel: i32) -> Self {
        Self {
            id,
            algorithm,
            channel,
            interactions: RwLock::new(Vec::with_capacity(5001)),
            agents: RwLock::new(Vec::with_capacity(100)),
        }
    }

    pub fn enqueue(
        &mut self,
        interaction_id: Uuid,
        queue_id: Uuid,
        priority: i32,
        core: &mut Core,
    ) -> Result<&str, &str> {
        Ok("enqueued")
    }
}
