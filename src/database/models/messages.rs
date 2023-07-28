use serde_json::{json, Value};

#[derive(sqlx::FromRow, Clone)]
pub struct MessageModelSql {
    pub id: String,
    pub thread_name: String,
    pub role: String,
    pub content: String,
}

pub struct GetMessageParams {
    pub thread_name: String,
}

pub struct CreateMessageBody {
    pub thread_name: String,
    pub role: String,
    pub content: String,
}

pub struct DeleteMessageParams {
    pub id: String,
}

impl MessageModelSql {
    pub fn coerce_to_value(&self) -> Value {
        json!({
            "role": self.role,
            "content": self.content,
        })
    }
}