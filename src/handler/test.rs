use ntex::{
    http::header,
    web::types::{Data, Json},
    web::{self, HttpResponse},
};
use serde::{Deserialize, Serialize};

use crate::data::state::AppState;
use crate::error::Result;


#[derive(Deserialize, Serialize, Debug)]
pub struct Uinfo {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeywordDoc {
    pub id: i32,
    pub keyword: String,
}

#[web::post("/keyword")]
async fn keyword(data: Data<AppState>, form: Json<KeywordDoc>) -> Result<HttpResponse> {
    data.insert_keyword_entry(&form).await?;
    Ok(HttpResponse::Ok()
    .header(header::CONTENT_TYPE, "application/json")
    .finish())
}

#[web::get("/")]
async fn index(data: Data<AppState>) -> Result<HttpResponse> {
    let content = data.find_keywords().await?;
    Ok(HttpResponse::Ok().json(&content))
}
