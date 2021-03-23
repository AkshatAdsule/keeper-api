use mongodb::bson::{self, doc, Bson, Document};
extern crate mongodb;

use serde::{Deserialize, Serialize};

pub mod db {
    use crate::Note;
    use mongodb::bson::{self, doc, Bson, Document};
    use mongodb::results::{DeleteResult, InsertOneResult};
    use mongodb::Collection;
    use std::env;
    use std::error::Error;
    use tokio::stream::StreamExt;

    pub struct DBService {
        client: mongodb::Client,
    }

    impl DBService {
        pub async fn new() -> Result<DBService, Box<dyn Error>> {
            let client_uri =
                env::var("MONGO_URI").expect("You must set the MONGO_URI environment var!");
            let service = DBService {
                client: mongodb::Client::with_uri_str(client_uri.as_ref()).await?,
            };
            Ok(service)
        }

        pub async fn create(&self, note: &Note) -> Result<InsertOneResult, Box<dyn Error>> {
            let notes = self.client.database("keeper").collection("notes");
            let serialized_note = bson::to_bson(note)?;
            let document = serialized_note.as_document().unwrap();

            Ok(notes.insert_one(document.to_owned(), None).await?)
        }

        pub async fn get_all(&self) -> Result<Vec<Note>, Box<dyn Error>> {
            let notes_collection = self.client.database("keeper").collection("notes");
            let mut cursor = notes_collection.find(None, None).await?;
            let mut data: Vec<Note> = Vec::new();

            while let Some(doc) = cursor.next().await {
                data.push(Note::from_doc(doc?));
            }
            Ok(data)
        }

        pub async fn delete(&self, id: &str) -> Result<DeleteResult, Box<dyn Error>> {
            let id = bson::oid::ObjectId::with_string(id)?;
            let notes_collection = self.client.database("keeper").collection("notes");
            let delete_res = notes_collection.delete_one(doc! {"_id": id}, None).await?;
            Ok(delete_res)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub title: String,
    pub body: String,
}

impl Note {
    fn from_doc(document: Document) -> Note {
        let mut _id = None;
        let mut _title = "".to_string();
        let mut _body = "".to_string();

        if let Some(&Bson::ObjectId(ref id)) = document.get("_id") {
            _id = Some(id.to_owned());
        }

        if let Some(&Bson::String(ref title)) = document.get("title") {
            _title = title.to_string();
        }
        if let Some(&Bson::String(ref body)) = document.get("body") {
            _body = body.to_string();
        }

        Note {
            id: _id,
            title: _title,
            body: _body,
        }
    }
}
