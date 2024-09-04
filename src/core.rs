use crate::agent::{Agent, AgentQueueMap};
use crate::interaction::Interaction;
use crate::queue::Queue;
use deadpool_postgres::tokio_postgres::types::Type;
use deadpool_postgres::Pool;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct Core {
    pool: Pool,
    pub queues: RwLock<HashMap<Uuid, RwLock<Queue>>>,
    pub agents: RwLock<HashMap<Uuid, Arc<RwLock<Agent>>>>,
    pub interactions: RwLock<HashMap<Uuid, Arc<RwLock<Interaction>>>>,
}

impl Core {
    pub async fn new(pool: Pool) -> Self {
        let queues = RwLock::new(HashMap::with_capacity(100));
        let agents = RwLock::new(HashMap::with_capacity(100));
        let interactions = RwLock::new(HashMap::with_capacity(100));
        let client = pool.get().await.unwrap();

        let rows = client.query("SELECT * FROM queues", &[]).await.unwrap();
        for row in rows.iter() {
            queues.write().unwrap().insert(
                row.get("id"),
                RwLock::new(Queue::new(
                    row.get("id"),
                    row.get("algorithm"),
                    row.get("channel"),
                )),
            );
        }

        let _assigned_queues_statement = client
            .prepare_typed(
                "SELECT * FROM agents_to_queues_assignment WHERE agent_id=$1",
                &[Type::UUID],
            )
            .await
            .unwrap();

        let rows = client.query("SELECT * FROM agents", &[]).await.unwrap();
        for row in rows.iter() {
            let agent_id: Uuid = row.get("uuid");
            let _queues = client
                .query(&_assigned_queues_statement, &[&agent_id])
                .await
                .unwrap();

            let mut assigned_queues: HashMap<Uuid, AgentQueueMap> = HashMap::new();

            for _queue in _queues.iter() {
                assigned_queues
                    .insert(row.get("id"), AgentQueueMap::new(row.get("priority"), true));
            }

            agents.write().unwrap().insert(
                row.get("id"),
                Arc::new(RwLock::new(Agent::new(row.get("id"), assigned_queues))),
            );
        }
        Self {
            pool,
            queues,
            agents,
            interactions,
        }
    }

    pub fn agent_exist(&self, id: &Uuid) -> bool {
        self.agents.read().unwrap().contains_key(id)
    }
    pub fn queue_exist(&self, id: &Uuid) -> bool {
        self.queues.read().unwrap().contains_key(id)
    }

    pub fn interaction_exist(&self, id: &Uuid) -> bool {
        self.interactions.read().unwrap().contains_key(id)
    }
}
