use deadpool_postgres::tokio_postgres::types::Type;
use deadpool_postgres::Pool;
use queue::Queue;
use std::collections::HashMap;
use std::hash::Hash;
use uuid::Uuid;

mod queue;

pub struct Core {
    pool: Pool,
    pub agents: HashMap<Uuid, Agent>,
    pub interactions: HashMap<Uuid, Interaction>,
    pub interactions_queues: HashMap<Uuid, Queue>,
    pub agents_queues: HashMap<Uuid, Queue>,
}

#[derive(Debug)]
pub struct Agent {
    id: Uuid,
    queues: HashMap<Uuid, i32>,
    state: i32,
}
#[derive(Debug)]
pub struct Interaction {
    id: Uuid,
    queues: HashMap<Uuid, i32>,
    state: i32,
}

impl Core {
    pub fn new(pool: Pool) -> Self {
        Self {
            pool,
            agents: HashMap::with_capacity(100),
            interactions: HashMap::with_capacity(100),
            interactions_queues: HashMap::with_capacity(100),
            agents_queues: HashMap::with_capacity(100),
        }
    }

    pub async fn init(&mut self) {
        let client = self.pool.get().await.unwrap();

        let _queues = client.query("SELECT * FROM queues", &[]).await.unwrap();
        for row in _queues.iter() {
            self.interactions_queues.insert(
                row.get("id"),
                Queue::new(row.get("id"), row.get("algorithm"), row.get("channel")),
            );
            self.agents_queues.insert(
                row.get("id"),
                Queue::new(row.get("id"), row.get("algorithm"), row.get("channel")),
            );
        }

        let _assigned_queues_statement = client
            .prepare_typed(
                "SELECT * FROM agents_to_queues_assignment WHERE agent_id=$1",
                &[Type::UUID],
            )
            .await
            .unwrap();

        let _agents = client.query("SELECT * FROM agents", &[]).await.unwrap();
        for _agent in _agents.iter() {
            let agent_id: Uuid = _agent.get("uuid");
            let _queues = client
                .query(&_assigned_queues_statement, &[&agent_id])
                .await
                .unwrap();

            let mut assigned_queues: HashMap<Uuid, i32> = HashMap::new();
            _queues.iter().for_each(|row| {
                assigned_queues.insert(row.get("id"), row.get("priority"));
            });
            self.agents.insert(
                _agent.get("id"),
                Agent::new(_agent.get("id"), assigned_queues),
            );
        }
    }

    pub fn queue_exist(&self, id: &Uuid) -> bool {
        match self.interactions_queues.get(id) {
            None => false,
            Some(_) => true,
        }
    }
}

impl Interaction {
    pub fn new(id: Uuid, state: i32) -> Self {
        Self {
            id,
            queues: HashMap::with_capacity(10),
            state,
        }
    }
}
impl Agent {
    fn new(id: Uuid, queues: HashMap<Uuid, i32>) -> Self {
        Self {
            id,
            queues,
            state: -1,
        }
    }
}
