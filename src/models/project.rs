use chrono::{DateTime, Utc};
use mongodb::bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Watermark {
    // Add watermark fields as needed - you'll need to specify the exact structure
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FacebookCredential {
    pub app_id: String,
    pub app_secret: String,
    pub access_token: String,
    pub ad_account_id: String,
    pub account_suffix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<Watermark>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub telegram_chat_id: Option<String>,
    pub facebook_credentials: HashMap<String, FacebookCredential>,
    pub package: Option<Package>,
    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    
    pub is_active: bool,
    pub is_logging: bool,
    
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
} 