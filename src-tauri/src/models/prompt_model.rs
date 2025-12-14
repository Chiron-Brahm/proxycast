use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub app_type: String,
    pub name: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub is_current: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl Prompt {
    pub fn new(id: String, app_type: String, name: String, content: String) -> Self {
        Self {
            id,
            app_type,
            name,
            content,
            description: None,
            is_current: false,
            created_at: Some(chrono::Utc::now().timestamp()),
        }
    }
}
