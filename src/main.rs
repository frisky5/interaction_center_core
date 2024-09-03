use crate::core::Core;
use crate::interactions_services::create_interaction;
use crate::queues_service::enqueue_interaction;
use actix_web::{web, App, HttpServer, Responder};
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{ManagerConfig, RecyclingMethod, Runtime};
use std::sync::RwLock;

mod agent;
mod contants;
mod core;
mod interaction;
mod interactions_services;
mod queue;
mod queues_service;
mod requests;
mod responses;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut config = deadpool_postgres::Config::new();
    config.user = Some(String::from("ic"));
    config.password = Some(String::from("Hala_1994"));
    config.dbname = Some(String::from("ic"));
    config.host = Some(String::from("postgresql.laudatur.local"));
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let shared_pool = web::Data::new(pool.clone());
    let core = web::Data::new(RwLock::new(Core::new(pool.clone()).await));
    HttpServer::new(move || {
        App::new()
            .app_data(shared_pool.clone())
            .app_data(core.clone())
            .service(
                web::scope("/core")
                    .service(enqueue_interaction)
                    .service(create_interaction),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
