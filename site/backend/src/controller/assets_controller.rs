use crate::db::assets_handler;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    token_address: String,
    token_id: i32,
}

#[get("/asset/asset")]
pub async fn get_asset(params: web::Query<Params>) -> actix_web::Result<impl Responder> {
    return match assets_handler::get_asset_data_for_token_address_and_token_id(
        &params.token_address,
        &params.token_id,
    )
    .await
    {
        None => Ok(HttpResponse::NotFound().finish()),
        Some(asset) => Ok(HttpResponse::Ok().json(asset)),
    };
}