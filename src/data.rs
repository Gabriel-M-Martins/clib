use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Snippet {
    pub command: String,
    pub description: String,
    pub category: Option<String>,
}