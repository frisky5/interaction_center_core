use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Agent {
    id: Uuid,
    queues: HashMap<Uuid, AgentQueueMap>,
    state: i32,
}

#[derive(Debug)]
pub struct AgentQueueMap {
    priority: i32,
    state: bool,
}
impl Agent {
    pub fn new(id: Uuid, queues: HashMap<Uuid, AgentQueueMap>) -> Self {
        Self {
            id,
            queues,
            state: -1,
        }
    }
}

impl AgentQueueMap {
    pub fn new(priority: i32, state: bool) -> Self {
        Self { priority, state }
    }
}
