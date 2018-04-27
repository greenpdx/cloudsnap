use actix::*;
use actix_web::*;
use chrono::{Utc, NaiveDateTime};
use diesel::sql_types::*;
use utils::schema::theme;
use utils::schema::comment;
use model::response::{ThemeListMsgs, ThemeAndCommentMsgs, Msgs};

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

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Comment {
    pub id: i32,
    pub theme_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="comment"]
pub struct NewComment<'a> {
    pub theme_id: i32,
    pub user_id: i32,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeNew {
    pub user_id: i32,
    pub category: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeId {
    pub theme_id: i32,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeComment {
    pub user_id: i32,
    pub the_theme_id: String,
    pub comment: String,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeList;

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct ThemeListResult {
    pub id: i32,
    pub user_id: i32,
    pub category: String,
    pub status: i32,
    pub title: String,
    pub content: String,
    pub view_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub rtime: String,
}

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct CommentReturn {
    pub id: i32,
    pub theme_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub rtime: String,
}

impl Message for ThemeList {
    type Result = Result<ThemeListMsgs, Error>;
}

impl Message for ThemeId {
    type Result = Result<ThemeAndCommentMsgs, Error>;
}

impl Message for ThemeNew {
    type Result = Result<Msgs, Error>;
}
impl Message for ThemeComment {
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

pub fn no_comment() -> CommentReturn {
    CommentReturn {
        id: 0,
        theme_id: 0,
        user_id: 0,
        content: "".to_owned(),
        created_at: Utc::now().naive_utc(),
        username: "".to_owned(),
        rtime: "".to_owned(),
    }
}

pub fn themelist() -> ThemeListResult {
    ThemeListResult {
        id: 0,
        user_id: 0,
        category: "".to_string(),
        status: 0,
        title: "".to_string(),
        content: "".to_string(),
        view_count: 0,
        comment_count: 0,
        created_at: Utc::now().naive_utc(),
        username: "".to_string(),
        rtime: "".to_string(),
    }
}

pub fn commen_return() -> CommentReturn {
    CommentReturn {
        id: 0,
        theme_id: 0,
        user_id: 0,
        content: "".to_string(),
        created_at: Utc::now().naive_utc(),
        username: "".to_string(),
        rtime: "".to_string(),
    }
}