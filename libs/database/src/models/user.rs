use std::ptr::eq;

use diesel::{prelude::*,result::Error};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};

use crate::{database::Database};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
} 

impl Database {
    pub fn signup(&mut self, name: String, email: String, password_hash: String) -> Result<String, Error>{
        
        
       let id = Uuid::new_v4();

       let u = User{
        id,
        name,
        email,
        password_hash,
        created_at: Utc::now().naive_utc(),
        updated_at:Utc::now().naive_utc()
       };

       diesel::insert_into(crate::schema::users::table)
       .values(&u)
       .returning(User::as_returning())
       .get_result(&mut self.conn)?;

       Ok(id.to_string())

    }

    pub fn signin(&mut self, name: String, input_email: String, input_password_hash: String) -> Result<bool, Error>{
        use crate::schema::users::dsl::*;
        
        let user_result = users
        .filter(email.eq(input_email))
        .select(User::as_select())
        .first(&mut self.conn)?;

        if user_result.password_hash != input_password_hash {
            return Ok(false);
        }

        Ok(false)
    }
}