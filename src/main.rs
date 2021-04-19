use ntex::{
    http::header,
    web::{self, App},
};
use ntex_cors::Cors;
use mongodb::Client;


mod data;
mod error;
mod handler;
mod model;
mod service;

use self::data::state::AppState;
use self::error::Result;
use self::handler::test::{index, keyword};
use self::handler::auth::signup;

#[ntex::main]
async fn main() -> Result<()> {
    let client = Client::with_options(Default::default())?;
    web::server( move || App::new()
        .data(AppState {
            client: client.clone(),
        })
        .wrap(
            Cors::new() 
              .allowed_methods(vec!["GET", "POST", "PUT", "DELETE","OPTIONS"])
              .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
              .max_age(3600)
              .finish())
              .service((signup, index, keyword))
        )
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

        Ok(())
}