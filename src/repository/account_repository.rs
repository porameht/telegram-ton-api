use mongodb::{
    bson::{doc, oid::ObjectId, Document, from_document, to_document},
    Collection, Database,
    options::UpdateOptions,
};
use crate::models::account::Account;
use crate::error::ApiError;

#[derive(Clone)]
pub struct AccountRepository {
    collection: Collection<Document>,
}

impl AccountRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection("accounts"),
        }
    }

    pub async fn create(&self, account: Account) -> Result<Account, ApiError> {
        let doc = to_document(&account)?;
        let result = self.collection.insert_one(doc, None).await?;
        let id = result.inserted_id.as_object_id()
            .ok_or_else(|| ApiError::InternalServerError("Failed to get inserted ID".into()))?;
        self.get_by_id(&id).await
    }

    pub async fn update(&self, id: &ObjectId, account: Account) -> Result<Account, ApiError> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": to_document(&account)?
        };

        let options = UpdateOptions::default();
        match self.collection.update_one(filter, update, options).await {
            Ok(result) if result.modified_count == 1 => self.get_by_id(id).await,
            Ok(_) => Err(ApiError::NotFound),
            Err(e) => Err(ApiError::MongoDB(e)),
        }
    }

    pub async fn delete(&self, id: &ObjectId) -> Result<bool, ApiError> {
        let filter = doc! { "_id": id };
        let result = self.collection.delete_one(filter, None).await?;
        Ok(result.deleted_count > 0)
    }

    pub async fn get_by_id(&self, id: &ObjectId) -> Result<Account, ApiError> {
        let filter = doc! { "_id": id };
        println!("Filter: {:?}", filter);
        let doc = self.collection.find_one(filter, None).await?
            .ok_or(ApiError::NotFound)?;
        Ok(from_document(doc)?)
    }

    pub async fn get_all(&self) -> Result<Vec<Account>, ApiError> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut accounts = Vec::new();
        
        while cursor.advance().await? {
            let raw_doc = cursor.current();
            if let Ok(doc) = Document::from_reader(raw_doc.as_bytes()) {
                match from_document(doc) {
                    Ok(account) => accounts.push(account),
                    Err(e) => eprintln!("Error deserializing account: {}", e),
                }
            }
        }
        Ok(accounts)
    }
}