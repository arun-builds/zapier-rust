use serde_json::Value as JsonValue;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::Insertable;
use database::schema::trigger as trigger_schema;
use diesel::sql_types::Jsonb;


#[derive(Deserialize,Serialize,Debug)]
pub struct Action {
    pub available_action_id: Uuid,
    pub sorting_order: i32,
    pub action_metadata: Option<JsonValue>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct ZapCreateSchema {
    pub available_trigger_id: Uuid,
    pub trigger_metadata: Option<JsonValue>,
    pub actions: Option<Vec<Action>>,
}

