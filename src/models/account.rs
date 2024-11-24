use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub wallet_address: String,
    pub email: String,
    pub account_name: String,
    #[serde(default)]
    pub project_ids: Vec<ObjectId>,
    
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
} 