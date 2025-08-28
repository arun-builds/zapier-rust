use crate::schema::zap_run_outbox;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = zap_run_outbox)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ZapRunOutbox {
    pub id: String,
    pub zap_run_id: String,
}
