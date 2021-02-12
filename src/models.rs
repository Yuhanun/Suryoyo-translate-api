use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct TranslationEntry {
    pub translation_id: i64,
    pub translation_keyword: String,
    pub translation_result: Option<serde_json::Value>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "translations"]
pub struct NewTranslation {
    pub translation_keyword: String,
    pub translation_result: Option<serde_json::Value>,
}
