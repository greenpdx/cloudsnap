use model::user::User;
use model::theme::{Theme, ThemeListResult};

pub enum MyError {
    NotFound,
    DatabaseError,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct Msgs {
    pub status: i32,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct SigninMsgs {
    pub status: i32,
    pub token: String,
    pub signin_user: User,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ThemeListMsgs {
    pub status: i32,
    pub message : String,
    pub theme_list: Vec<ThemeListResult>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct UserInfoMsgs {
    pub status: i32,
    pub message : String,
    pub current_user: User,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct ThemeMsgs {
    pub status: i32,
    pub message : String,
    pub theme : Theme,
    pub theme_user : User,
}
 