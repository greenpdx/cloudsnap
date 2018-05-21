use model::user::User;
use model::theme::{ThemeListResult, Theme, CommentReturn};
use model::community::{Community, CommunityThemeListResult};

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
pub struct ThemeAndCommentsMsgs {
    pub theme: Theme,
    pub theme_user: User,
    pub theme_community_name: String,
    pub theme_rtime: String,
    pub theme_comments: Vec<CommentReturn>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CommunityNamesMsgs {
    pub status: i32,
    pub message : String,
    pub community_names : Vec<String>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CommunityCategorysMsgs {
    pub status: i32,
    pub message : String,
    pub community_categorys : Vec<String>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CommunitysMsgs {
    pub status: i32,
    pub message : String,
    pub communitys : Vec<Community>,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct CommunityThemesMsgs {
    pub status: i32,
    pub message : String,
    pub community_theme_list : Vec<CommunityThemeListResult>,
}

impl ThemeAndCommentsMsgs {
    pub fn new() -> ThemeAndCommentsMsgs {
            ThemeAndCommentsMsgs{
                theme: Theme::new(),
                theme_user: User::new(),
                theme_community_name: "".to_string(),
                theme_rtime: "".to_string(),
                theme_comments: vec![],
            }
    }
}