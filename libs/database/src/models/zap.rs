use diesel::{prelude::*};
use serde_json::Value as JsonValue;
use serde::{Serialize, Deserialize};
use uuid::Uuid;




#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::zap)]
pub struct Zap {
    pub id: Uuid,
    pub trigger_id: Uuid,
    pub user_id: Uuid,
} 

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]

#[diesel(table_name = crate::schema::trigger)]
pub struct Trigger {
    pub id: Uuid,
    pub zap_id: Uuid,
    pub trigger_id: Uuid,
    pub metadata: JsonValue,
} 

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::action)]
pub struct Action {
    pub id: Uuid,
    pub zap_id: Uuid,
    pub action_id: Uuid,
    pub metadata: JsonValue,
    pub sorting_order: i32,
} 