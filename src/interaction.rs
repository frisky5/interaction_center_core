use crate::contants::InteractionState;
use crate::contants::InteractionState::{
    Closed, DequeuedByManualRemoval, DequeuedToPresent, Enqueued, New, Presenting, Rejected,
};
use std::collections::HashMap;
use uuid::Uuid;
use InteractionState::{Accepted, ReEnqueuedDueToRejection};

#[derive(Debug)]
pub struct Metadata {
    priority: i32,
}

#[derive(Debug)]
pub struct Interaction {
    id: Uuid,
    queues: HashMap<Uuid, Metadata>,
    state: InteractionState,
}
impl Interaction {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            queues: HashMap::new(),
            state: New,
        }
    }

    pub fn add_queue(&mut self, id: Uuid, priority: i32) {
        self.queues.insert(id, Metadata { priority });
    }

    pub fn is_in_queue(&self, id: &Uuid) -> bool {
        self.queues.contains_key(id)
    }

    pub fn is_state_allowed(&self, new_state: InteractionState) -> bool {
        match new_state {
            New => false,
            Enqueued => [New, Enqueued, DequeuedByManualRemoval].contains(&self.state),
            DequeuedToPresent => [Enqueued].contains(&self.state),
            DequeuedByManualRemoval => [Enqueued].contains(&self.state),
            Closed => [New, Enqueued].contains(&self.state),
            Presenting => [DequeuedToPresent].contains(&self.state),
            Rejected => [Presenting].contains(&self.state),
            Accepted => [Presenting].contains(&self.state),
            ReEnqueuedDueToRejection => [Rejected].contains(&self.state),
        }
    }
}
