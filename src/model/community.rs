use actix::*;
use actix_web::*;
use utils::schema::communitys;
use chrono::{Utc, NaiveDateTime};
use model::response::{CommunitysMsgs,CommunityNamesMsgs, CommunityCategorysMsgs, Msgs, CommunityThemesMsgs};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Community {
    pub id: i32,
    pub create_user_id: i32,
    pub community_name: String,
    pub community_category: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,Insertable,Debug, Clone)]
#[table_name="communitys"]
pub struct NewCommunity<'a> {
    pub create_user_id: i32,
    pub community_name: &'a str,
    pub community_category: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CommunityNew {
    pub create_user_id: i32,
    pub community_name: String,
    pub community_category: String,
}
#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CommunityNames {
    pub create_user_id: i32,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CommunityCategorys;

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct Communitys;

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CommunityThemes {
    pub community_name: String,
}


#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CommunityLike {
    pub user_id: i32,
    pub community_name: String,
}

#[derive(Deserialize,Serialize, Debug, Clone)]
pub struct CommunityThemeListResult {
    pub id: i32,
    pub user_id: i32,
    pub community_id: i32,
    pub theme_status: i32,
    pub title: String,
    pub content: String,
    pub view_count: i32,
    pub comment_count: i32,
    pub created_at: NaiveDateTime,
    pub username: String,
    pub rtime: String,
}


impl Message for CommunityThemes {
    type Result = Result<CommunityThemesMsgs, Error>;
}
impl Message for CommunityNew {
    type Result = Result<Msgs, Error>;
}

impl Message for CommunityNames {
    type Result = Result<CommunityNamesMsgs, Error>;
}

impl Message for CommunityCategorys {
    type Result = Result<CommunityCategorysMsgs, Error>;
}
impl Message for Communitys {
    type Result = Result<CommunitysMsgs, Error>;
}

impl Message for CommunityLike {
    type Result = Result<Msgs, Error>;
}

impl CommunityThemeListResult {
    pub fn new () -> CommunityThemeListResult {
        CommunityThemeListResult {
            id: 0,
            user_id: 0,
            community_id: 0,
            theme_status: 0,
            title: "".to_string(),
            content: "".to_string(),
            view_count: 0,
            comment_count: 0,
            created_at: Utc::now().naive_utc(),
            username: "".to_string(),
            rtime: "".to_string(),
        }
    }
}
