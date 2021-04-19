use mongodb::{options::FindOptions, Client};
use mongodb::bson::{self, doc, Bson};
use futures::TryStreamExt;
use crate::error::Result;
use crate::handler::test::KeywordDoc;

pub struct AppState {
    pub client: Client,
}

impl AppState {
    pub async fn find_keywords(&self) -> Result<Vec<KeywordDoc>> {
        let options = FindOptions::builder()
            .sort(doc! { "keyword": 1 })
            .limit(50)
            .build();
            
        let cursor = self
            .client
            .database("across-travel")
            .collection("info")
            .find(None, options)
            .await?;
        
        let keywords = cursor
            .and_then(|doc| {
                let doc: KeywordDoc = match bson::from_bson(Bson::Document(doc)) {
                    Ok(doc) => doc,
                    Err(e) => return futures::future::err(e.into()),
                };

                futures::future::ok(doc)
            })
            .try_collect()
            .await?;
        Ok(keywords)
    }

    pub async fn insert_keyword_entry(&self, entry: &KeywordDoc) -> Result<()> {
        let keywords = doc! {
                "id": entry.id,
                "keyword": entry.keyword.to_lowercase(),
            };

        self.client
            .database("across-travel")
            .collection("info")
            .insert_one(keywords, None)
            .await?;

        Ok(())
    }
}