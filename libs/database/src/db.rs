use crate::{config::Config, schema::action::table, schema::zap_run, schema::zap_run_outbox};
use diesel::prelude::*;
use diesel::sql_types::Jsonb;
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub struct Database {
    pub conn: PgConnection,
}

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::zap_run)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ZapRun {
    pub id: String,
    pub zap_id: String,
    #[diesel(sql_type = Jsonb)]
    pub metadata: JsonValue,
}

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::zap_run_outbox)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ZapRunOutbox {
    pub id: String,
    pub zap_run_id: String,
}

impl Database {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn })
    }

    pub fn create_zap(
            &mut self,
            zap_id: String,
        metadata:JsonValue ,
    ) -> QueryResult<()> {
    
        

        self.conn.transaction(|connection| {
            let zap_run = ZapRun{
                id: Uuid::new_v4().to_string(),
                zap_id,
                metadata,
            };

            diesel::insert_into(zap_run::table)
            .values(&zap_run)
            .execute(connection)?;

            let zap_run_outbox = ZapRunOutbox{
                id: Uuid::new_v4().to_string(),
                zap_run_id: zap_run.id
            };

            diesel::insert_into(zap_run_outbox::table)
            .values(&zap_run_outbox)
            .execute(connection)?;

            diesel::result::QueryResult::Ok(())
        })
    }

    pub fn get_pending_zap_run_outbox(&mut self) -> QueryResult<Vec<ZapRunOutbox>> {
        
        let pending_rows = zap_run_outbox::table
        .limit(10)
        .select(ZapRunOutbox::as_select())
        .load(&mut self.conn)
        .expect("Error loading zap run outbox");

        Ok(pending_rows)
    }

  
}