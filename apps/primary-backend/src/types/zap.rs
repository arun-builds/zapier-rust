use serde_json::Value as JsonValue;
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize)]
pub struct Action {
    pub action_id: String,
    pub action_metadata: Option<JsonValue>,
}

#[derive(Deserialize,Serialize)]
pub struct ZapCreateSchema {
    pub trigger_id: String,
    pub trigger_metadata: Option<JsonValue>,
    pub actions: Vec<Action>,
}