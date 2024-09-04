#[derive(Debug, PartialEq, Eq)]
pub enum InteractionState {
    New,
    Presenting,
    Rejected,
    Accepted,
    Enqueued,
    ReEnqueuedDueToRejection,
    DequeuedToPresent,
    DequeuedByManualRemoval,
    Closed,
}

pub fn get_id(state: InteractionState) -> i32 {
    match state {
        InteractionState::New => 1_000,
        InteractionState::Presenting => 1_001,
        InteractionState::Rejected => 1_002,
        InteractionState::Accepted => 1_003,
        InteractionState::Enqueued => 2_000,
        InteractionState::ReEnqueuedDueToRejection => 2_001,
        InteractionState::DequeuedToPresent => 2_002,
        InteractionState::DequeuedByManualRemoval => 2_003,
        InteractionState::Closed => 9_000,
    }
}
