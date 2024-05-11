use actix_web::{middleware::Logger, web, App, HttpServer};
mod controllers;
mod models;
mod services;
use controllers::{balance_get_request, reset, transaction_request};
use models::AccountManager;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let account_manager  = web::Data::new(AccountManager::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(account_manager.clone())
            .service(transaction_request)
            .service(balance_get_request)
            .service(reset)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
