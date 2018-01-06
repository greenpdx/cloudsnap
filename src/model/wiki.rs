use utils::schema::wiki;
use chrono::{DateTime,Utc};

#[derive(Clone,Debug,Serialize,Queryable, Associations)]
pub struct Wiki {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub rtime: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="wiki"]
pub struct NewWiki<'a> {
    pub category: &'a str,
    pub title: &'a str,
    pub raw: &'a str,
    pub cooked: &'a str,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}