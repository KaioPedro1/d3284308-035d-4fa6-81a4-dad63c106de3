use actix_web::{ post, web, HttpResponse, Responder};

use crate::models::AccountManager;


#[post("/reset")]
pub async fn reset(acc_manager: web::Data<AccountManager>) -> impl Responder {
    acc_manager.clear_accounts();
    HttpResponse::Ok().body("OK")
}