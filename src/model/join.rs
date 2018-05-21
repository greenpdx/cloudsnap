use actix::*;
use actix_web::*;
use chrono::{Utc, NaiveDateTime};
use utils::schema::joins;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Join {
    pub id: i32,
    pub user_id: i32,
    pub user_role: String,
    pub community_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="joins"]
pub struct NewJoin<'a> {
    pub user_id: i32,
    pub user_role: &'a str,
    pub community_id: i32,
    pub created_at: NaiveDateTime,
}
