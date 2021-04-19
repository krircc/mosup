use ntex::{
    http::header,
    web::types::{Data, Json},
    web::{self, HttpResponse},
};
use chrono::Utc;
use mongodb::bson::doc;

use crate::data::state::AppState;
use crate::error::Result;
use crate::model::user::{User};

#[web::post("/signup")]
pub async fn signup( data: Data<AppState>, signup_user: Json<User>) -> Result<HttpResponse> {
        data.insert_signup_user(&signup_user).await?;

        Ok(HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "application/json")
        .finish())
}

impl AppState {

    pub async fn insert_signup_user(&self, signup_user: &User) -> Result<()> {
        let avatar_url = "http://www.gravatar.com/avatar/".to_string() + "?s=128&d=identicon";
        println!("{:?}",  Utc::now());
        let user_new = doc! {
                "username": signup_user.username.to_string(),
                "email": signup_user.email.to_string(),
                "password": signup_user.password.to_string(),
                "created_at": Utc::now(),
                // "created_at": Utc::now().timestamp_millis(),
                "avatar": avatar_url,
            };

        self.client
            .database("across-travel")
            .collection("userdata")
            .insert_one(user_new, None)
            .await?;

        Ok(())
    }
}
