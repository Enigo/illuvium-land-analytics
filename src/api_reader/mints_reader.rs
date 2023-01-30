use crate::api_reader::utils;
use log::{error, info};
use sqlx::{Pool, Postgres};

use crate::db::{db_handler, mints_handler};
use crate::model::mint::Mint;

const MINTS_URL: &str = "https://api.x.immutable.com/v1/mints?token_address=0x9e0d99b864e1ac12565125c5a82b59adea5a09cd&page_size=200";

pub async fn read() {
    let pool = db_handler::open_connection().await;
    let mut cursor = None;
    loop {
        cursor = execute_and_get_cursor(cursor, &pool).await;
        if cursor.is_none() {
            break;
        } else {
            info!("Current cursor: {}", cursor.clone().unwrap());
        }
    }
    db_handler::close_connection(pool).await;
}

async fn execute_and_get_cursor(cursor: Option<String>, pool: &Pool<Postgres>) -> Option<String> {
    let url = if cursor.is_some() {
        MINTS_URL.to_owned() + "&cursor=" + cursor.unwrap().as_str()
    } else {
        String::from(MINTS_URL)
    };
    let response = utils::fetch_api_response::<Mint>(url.as_str()).await;
    match response {
        Ok(mint) => {
            info!("Processing mint response");
            if !mint.result.is_empty() {
                mints_handler::save_mints(mint.result, pool).await;
            }

            if !mint.cursor.is_empty() {
                return Some(mint.cursor);
            }
            None
        }
        Err(e) => {
            error!("Mints API response cannot be parsed! {}", e);
            None
        }
    }
}