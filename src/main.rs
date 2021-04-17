use anyhow::Result;
use mongodb::{
    bson::{doc, Bson, Document},
    options::{ClientOptions, DropCollectionOptions, StreamAddress, UpdateModifications},
    Client, Collection, Database,
};
use chrono::NaiveDateTime;
use serde::{
    de::{DeserializeOwned, Error},
    Deserialize, Deserializer, Serialize,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Uinfo {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub avatar: String,
}


#[derive(Deserialize, Serialize, Debug)]
struct User {
    id: String,
    name: String,
    reviewed_book_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Book {
    id: String,
    name: String,
    reviews: Vec<Review>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Review {
    user_id: String,
    text: String,
}

//just for convinience.
fn s(s: &str) -> String {
    s.to_string()
}

async fn create_users(client: &Client) -> Result<()> {
    let db = client.database("demodb");
    let user_coll = db.collection::<User>("users");
    user_coll
        .insert_many(
            vec![
                User {
                    id: s("user_1"),
                    name: s("john"),
                    reviewed_book_ids: vec![],
                },
                User {
                    id: s("user_2"),
                    name: s("anna"),
                    reviewed_book_ids: vec![],
                },
            ],
            None,
        )
        .await?;

    user_coll
        .insert_one(
            User {
                id: s("user_3"),
                name: s("joseph"),
                reviewed_book_ids: vec![],
            },
            None,
        )
        .await?;

    Ok(())
}

async fn create_books(client: &Client) -> Result<()> {
    let db = client.database("demodb");
    let book_coll = db.collection::<Book>("books");
    book_coll
        .insert_one(
            Book {
                id: s("book_1"),
                name: s("The Hitchhiker's Guide to Somewhere"),
                reviews: vec![],
            },
            None,
        )
        .await?;

    Ok(())
}

async fn find_users(client: &Client) -> Result<()> {
    let db = client.database("demodb");
    let user_coll = db.collection::<User>("users");
    let found = user_coll.find_one(Some(doc! {"id":"user_1"}), None).await?;
    let found = found.unwrap();
    assert_eq!(s("user_1"), found.id);
    println!("\nfound user:{:?}", found);
    Ok(())
}

async fn add_reviews_in_session(client: &Client) -> Result<()> {
    let db = client.database("demodb");
    let user_coll = db.collection::<User>("users");
    let book_coll = db.collection::<Book>("books");

    let user_id = s("user_2");
    let book_id = s("book_1");
    let mut session = client.start_session(None).await?;
    {
        // TODO(tacogips) try to find a doc using indices.
        
        book_coll
            .update_one_with_session(
                doc! {"id" : book_id.clone()},
                UpdateModifications::Document(doc! {
                    "$push":{
                        "reviews":{
                            "user_id": user_id.clone(),
                            "text": s("Good reading")
                        },
                    }
                }),
                None,
                &mut session,
            )
            .await?;

        user_coll
            .update_one_with_session(
                doc! {"id" : user_id.clone()},
                UpdateModifications::Document(doc! {
                    "$push":{
                        "reviewed_book_ids":book_id.clone(),
                    }
                }),
                None,
                &mut session,
            )
            .await?;

        {
            // another session
            let mut another_session = client.start_session(None).await?;
            let found = book_coll
                .find_one_with_session(Some(doc! {"id":book_id}), None, &mut another_session)
                .await?
                .unwrap();

            // TODO(tacogips) Assertion error occures here. Is the push operations above supposed to be not visible from another session?
            assert_eq!(0, found.reviews.len());
            println!("\nupdated book:{:?}", found);

            let found = user_coll
                .find_one_with_session(Some(doc! {"id":user_id}), None, &mut another_session)
                .await?
                .unwrap();

            assert_eq!(0, found.reviewed_book_ids.len());
            println!("\nupdated user:{:?}", found);
        }
    }

    Ok(())
}

async fn drop_colls(client: &Client) -> Result<()> {
    let db = client.database("demodb");
    let user_coll = db.collection::<User>("users");
    user_coll.drop(None).await?;

    let book_coll = db.collection::<Book>("books");
    book_coll.drop(None).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let opts = ClientOptions::builder()
        .hosts(vec![
            StreamAddress {
                hostname: "mongo1".to_string(),
                port: Some(30001),
            },
            StreamAddress {
                hostname: "mongo2".to_string(),
                port: Some(30002),
            },
            StreamAddress {
                hostname: "mongo3".to_string(),
                port: Some(30003),
            },
        ])
        .repl_set_name(Some("my-replica-set".to_string()))
        .build();

    let client = Client::with_options(opts).unwrap();
    create_users(&client).await.unwrap();
    create_books(&client).await.unwrap();
    find_users(&client).await.unwrap();
    add_reviews_in_session(&client).await.unwrap();
    drop_colls(&client).await.unwrap();
}