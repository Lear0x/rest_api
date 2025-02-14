use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
}
