use keeper_api::db::DBService;
use std::error::Error;

// #[derive(Serialize, Deserialize, Debug)]
// struct Note {
//     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//     id: Option<bson::oid::ObjectId>,
//     title: String,
//     body: String
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    // let client_uri = env::var("MONGO_URI").expect("You must set the MONGO_URI environment var!");
    //
    // let client = mongodb::Client::with_uri_str(client_uri.as_ref()).await?;
    //
    // // Notes collection
    // let notes = client.database("keeper").collection("notes");
    // let note: Note =  Note{ id: None, title: "This was made with rust!!".to_owned(), body: "Rust is just amazing!".to_owned()};
    //
    // let serialized_note = bson::to_bson(&note)?;
    // let document = serialized_note.as_document().unwrap();
    //
    // let inserted_document = notes.insert_one(document.to_owned(), None).await?;
    //
    // let inserted_note_id = inserted_document
    //     .inserted_id
    //     .as_object_id()
    //     .expect("Retrieved _id should have been of type ObjectId");
    //
    // println!("Captain Marvel document ID: {:?}", inserted_note_id);

    let db = DBService::new().await?;
    let data = db.get_all().await?;
    let mut delete = true;

    for note in data {
        if delete {
            println!("Deleting note: {:?}\n", note);
            let del_res = db.delete(note.id.unwrap()).await?;
            println!("Deleted {} documents", del_res.deleted_count);
            delete = false;
            continue;
        }
        // println!("Got note: {:?}\n", note);
    }
    // let test_note = Note {id: None, title: "from dbService".to_owned(), body: "posted from dbService".to_owned()};
    // let res = db.create(&test_note).await?;
    // println!("Document ID: {}", res.inserted_id);

    Ok(())
}
