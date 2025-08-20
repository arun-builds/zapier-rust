use crate::{db::Database, schema::{available_action, available_trigger, zap, trigger, action}};
use diesel::prelude::*;
use uuid::Uuid;

#[test]
fn test_seed() {

    let mut db = Database::default().unwrap();
    // add_available_actions(&mut db);
    // add_available_triggers(&mut db);
    // create_zap(&mut db);
    // create_trigger(&mut db);
    // add_action_to_trigger(&mut db);
}

fn add_available_actions(db: &mut   Database){

    #[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::available_action)]
#[diesel(check_for_backend(diesel::pg::Pg))]
   pub struct AvailableAction{
    pub id: String,
    pub name: String,
   }

   let available_actions = AvailableAction{
    id: Uuid::new_v4().to_string(),
    name: "solana_send".to_string(),
   };

   diesel::insert_into(available_action::table)
   .values(&available_actions)
   .execute(&mut db.conn)
   .unwrap();

}

fn add_available_triggers(db: &mut Database){

    #[derive(Queryable, Insertable, Selectable)]
    #[diesel(table_name = crate::schema::available_trigger)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct AvailableTrigger{
        pub id: String,
        pub name: String,
    }

    let available_triggers = AvailableTrigger{
        id: Uuid::new_v4().to_string(),
        name: "webhooks".to_string(),
    };

    diesel::insert_into(available_trigger::table)
    .values(&available_triggers)
    .execute(&mut db.conn)
    .unwrap();

}

fn create_zap(db: &mut Database){

    #[derive(Queryable, Insertable, Selectable)]
    #[diesel(table_name = crate::schema::zap)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Zap{
        pub id: String,
        pub trigger_id: String,
    }

    let zap = Zap{
        id: Uuid::new_v4().to_string(),
        trigger_id: "default".to_string(),
    };

    diesel::insert_into(zap::table)
    .values(&zap)
    .execute(&mut db.conn)
    .unwrap();
}

fn create_trigger(db: &mut Database){

    #[derive(Queryable, Insertable, Selectable)]
    #[diesel(table_name = crate::schema::trigger)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Trigger{
        pub id: String,
        pub zap_id: String,
        pub trigger_id: String,
    }

    let trigger = Trigger{
        id: Uuid::new_v4().to_string(),
        zap_id: "66f4d254-6ca8-4b39-8840-e9a0dea9e2ba".to_string(),
        trigger_id: "80f58108-67b4-4878-8a6e-24b21f678185".to_string(),
    };

    diesel::insert_into(trigger::table)
    .values(&trigger)
    .execute(&mut db.conn)
    .unwrap();
}

fn add_action_to_trigger(db: &mut Database){

    #[derive(Queryable, Insertable, Selectable)]
    #[diesel(table_name = crate::schema::action)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Action{
        pub id: String,
        pub zap_id: String,
        pub action_id: String,
    }
    // send_solana action id => 0fe14193-1ae2-4ac2-b9d0-72781c1e1ee4
    // email action id => 9ee48120-5d9c-4695-b722-81ae3ccf7dc2
    let action = Action{
        id: Uuid::new_v4().to_string(),
        zap_id: "66f4d254-6ca8-4b39-8840-e9a0dea9e2ba".to_string(),
        action_id: "0fe14193-1ae2-4ac2-b9d0-72781c1e1ee4".to_string(),
    };

    diesel::insert_into(action::table)
    .values(&action)
    .execute(&mut db.conn)
    .unwrap();
}