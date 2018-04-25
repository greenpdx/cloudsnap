use actix::*;
use actix_web::*;
use chrono::Utc;
use diesel::sql_types::*;
use utils::schema::theme;
use chrono::NaiveDateTime;
use model::response::{ThemeListMsgs, ThemeMsgs, Msgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Theme {
    pub id: i32,
    pub user_id: i32,
    pub category: String,
    pub status: i32,
    pub title: String,
    pub content: String,
    pub view_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="theme"]
pub struct NewTheme<'a> {
    pub user_id: i32,
    pub category: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ThemeNew {
    pub user_id: i32,
    pub category: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ThemeId {
    pub theme_id: i32,
}

pub struct ThemeList;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable,QueryableByName)]
pub struct ThemeListResult {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub user_id: i32,
    #[sql_type = "Text"]
    pub category: String,
    #[sql_type = "Integer"]
    pub status: i32,
    #[sql_type = "Text"]
    pub title: String,
    #[sql_type = "Text"]
    pub content: String,
    #[sql_type = "Integer"]
    pub view_count: i32,
    #[sql_type = "Integer"]
    pub comment_count: i32,
    #[sql_type = "Timestamp"]
    pub created_at: NaiveDateTime,
    #[sql_type = "Text"]
    pub username: String,
}

impl Message for ThemeList {
    type Result = Result<ThemeListMsgs, Error>;
}

impl Message for ThemeId {
    type Result = Result<ThemeMsgs, Error>;
}

impl Message for ThemeNew {
    type Result = Result<Msgs, Error>;
}

pub fn no_theme() -> Theme {
    Theme {
        id: 0,
        user_id: 0,
        category: "".to_owned(),
        status: 0,
        title: "".to_owned(),
        content: "".to_owned(),
        view_count: 0,
        comment_count: 0,
        created_at: Utc::now().naive_utc(),
    }
}