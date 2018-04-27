use diesel;
use diesel::*;
use actix::*;
use actix_web::*;
use timeago;
use chrono::{Utc, Datelike, Timelike, NaiveDateTime};
use model::response::{ThemeListMsgs, ThemeAndCommentMsgs, Msgs};
use model::theme::{themelist, Theme, ThemeList, ThemeListResult, ThemeId, NewTheme, 
                   ThemeNew, Comment, CommentReturn, NewComment, ThemeComment, commen_return, no_theme, no_comment};
use model::db::ConnDsl;
use model::user::{User, no_user};
use utils::{time, markdown_to_html};

impl Handler<ThemeList> for ConnDsl {
    type Result = Result<ThemeListMsgs, Error>;

    fn handle(&mut self, theme_list: ThemeList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::theme::dsl::*;
        use utils::schema::users;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let mut themes = theme.load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let mut themes_list: Vec<ThemeListResult> = vec![];
        for theme_one in themes {
            let mut themes_list_one = themelist();
            themes_list_one.id = theme_one.id;
            themes_list_one.user_id = theme_one.user_id;
            themes_list_one.category = theme_one.category;
            themes_list_one.status = theme_one.status;
            themes_list_one.title = theme_one.title;
            themes_list_one.content = theme_one.content;
            themes_list_one.view_count = theme_one.view_count;
            themes_list_one.comment_count = theme_one.comment_count;
            themes_list_one.created_at = theme_one.created_at;
            let rtime = time( Utc::now().naive_utc(), theme_one.created_at);
            themes_list_one.rtime = rtime;
            let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
            match theme_user {
                Some(user) => {
                        themes_list_one.username = user.username;
                },
                None => {
                        themes_list_one.username = "".to_string();
                },
            }
            {
                themes_list.push(themes_list_one)
            }
        }
        Ok(ThemeListMsgs { 
            status: 200,
            message : "theme_list result.".to_string(),
            theme_list: themes_list,
        })
    }
}

impl Handler<ThemeId> for ConnDsl {
    type Result = Result<ThemeAndCommentMsgs, Error>;

    fn handle(&mut self, theme_id: ThemeId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::theme::dsl::*;
        use utils::schema::users;
        use utils::schema::comment;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::update(theme).filter(&id.eq(&theme_id.theme_id)).set((view_count.eq(view_count + 1),)).execute(conn).map_err(error::ErrorInternalServerError)?;
        let the_theme =  theme.filter(&id.eq(&theme_id.theme_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let mut theme_comment = comment::table.filter(&comment::theme_id.eq(&theme_id.theme_id)).load::<Comment>(conn).map_err(error::ErrorInternalServerError)?;
        let mut comment_list: Vec<CommentReturn> = vec![];
        for comment in &mut theme_comment {
            let mut comment_list_one = commen_return();
            comment_list_one.id = comment.id;
            comment_list_one.theme_id = comment.theme_id;
            comment_list_one.user_id = comment.user_id;
            comment_list_one.content = markdown_to_html(&comment.content);
            comment_list_one.created_at = comment.created_at;
            let comment_user = users::table.filter(&users::id.eq(comment.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
            match comment_user {
                    Some(someuser) => {  comment_list_one.username = someuser.username; },
                    None => { comment_list_one.username = "".to_string(); },
            }
            let rtime = time(Utc::now().naive_utc(), comment.created_at);
            comment_list_one.rtime = rtime;
            comment_list.push(comment_list_one);
        }
        let no_theme = no_theme();
        let no_user = no_user();
        let no_comment = no_comment();
        let no_comments = vec![];
        match the_theme {
            Some(mut themeid) => {
                let theme_rtime = time(Utc::now().naive_utc(), themeid.created_at);
                themeid.content = markdown_to_html(&themeid.content);
                let uid = themeid.user_id;
                let user_result = users::table.filter(&users::id.eq(uid)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match user_result {
                    Some(themeid_user) => {
                        Ok(ThemeAndCommentMsgs { 
                            status: 200,
                            message : "The  theme info.".to_string(),
                            theme: themeid,
                            theme_user: themeid_user,
                            theme_comment: comment_list,
                            theme_rtime : theme_rtime,
                        })
                    },
                    None => {
                        Ok(ThemeAndCommentMsgs { 
                            status: 400,
                            message : "error.".to_string(),
                            theme: no_theme,
                            theme_user: no_user,
                            theme_comment: no_comments,
                            theme_rtime : "".to_string(),
                        })
                    },
                }
                    
            },
            None => {
                    Ok(ThemeAndCommentMsgs { 
                        status: 400,
                        message : "error.".to_string(),
                        theme: no_theme,
                        theme_user: no_user,
                        theme_comment: no_comments,
                        theme_rtime : "".to_string(),
                    })
            },
        }
    }
}

impl Handler<ThemeNew> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, theme_new: ThemeNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::theme::dsl::*;

        let new_theme = NewTheme {
            user_id: theme_new.user_id,
            category: &theme_new.category,
            title: &theme_new.title,
            content: &theme_new.content,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(theme).values(&new_theme).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                    status: 200,
                    message : "Theme Publish Successful.".to_string(),
        })        
    }
}

impl Handler<ThemeComment> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, theme_comment: ThemeComment, _: &mut Self::Context) -> Self::Result {
        use utils::schema::comment::dsl::*;
        use utils::schema::theme;
        let the_theme_id: i32 = theme_comment.the_theme_id.to_owned().parse().map_err(error::ErrorBadRequest)?;
        let new_comment = NewComment {
            theme_id: the_theme_id,
            user_id: theme_comment.user_id,
            content: &theme_comment.comment,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(comment).values(&new_comment).execute(conn).map_err(error::ErrorInternalServerError)?;
        diesel::update(theme::table)
            .filter(&theme::id.eq(&the_theme_id))
            .set((
                theme::comment_count.eq(theme::comment_count + 1),
            )).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                    status: 200,
                    message : "Comment Add Successful.".to_string(),
        })        
    }
}