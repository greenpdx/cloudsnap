use diesel;
use diesel::*;
use actix::*;
use actix_web::*;
use timeago;
use chrono::{Utc, Datelike, Timelike, NaiveDateTime};
use model::response::{ThemeListMsgs, Msgs, ThemeAndCommentsMsgs};
use model::theme::{Theme, ThemeList, ThemeListResult, ThemeId, NewTheme, ThemeNew, Comment, CommentReturn, NewComment, ThemeComment};
use model::community::Community;
use model::db::ConnDsl;
use model::user::User;
use model::join::Join;
use utils::{time, markdown_to_html};

impl Handler<ThemeList> for ConnDsl {
    type Result = Result<ThemeListMsgs, Error>;

    fn handle(&mut self, theme_list: ThemeList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::communitys;
        use utils::schema::joins;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        if theme_list.user_id == 0 {
            let mut themes_result = themes.order(id).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
            let mut themes_list: Vec<ThemeListResult> = vec![];
            for theme_one in themes_result {
                let mut themes_list_one = ThemeListResult::new();
                themes_list_one.id = theme_one.id;
                themes_list_one.user_id = theme_one.user_id;
                themes_list_one.community_id = theme_one.community_id;
                themes_list_one.theme_status = theme_one.theme_status;
                themes_list_one.title = theme_one.title;
                themes_list_one.content = theme_one.content;
                themes_list_one.view_count = theme_one.view_count;
                themes_list_one.comment_count = theme_one.comment_count;
                themes_list_one.created_at = theme_one.created_at;
                let rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                themes_list_one.rtime = rtime;
                let community_result =  communitys::table.filter(communitys::id.eq(theme_one.community_id)).load::<Community>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match community_result {
                    Some(community_one) => { themes_list_one.community_name = community_one.community_name; },
                    None => { println!("No community result");},
                }
                let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match theme_user {
                    Some(user) => { themes_list_one.username = user.username;},
                    None => { println!("No theme_user result");},
                }
                themes_list.push(themes_list_one);
            }
            Ok(ThemeListMsgs { 
                status: 200,
                message : "theme_list result.".to_string(),
                theme_list: themes_list,
            })
        }
        else{
            let join_community_ids = joins::table.filter(joins::user_id.eq(theme_list.user_id)).load::<Join>(conn).map_err(error::ErrorInternalServerError)?;
            let mut themes_result_user_id: Vec<Theme> = vec![];
            for join_community_id_one in join_community_ids {
                let theme_community_id = themes.filter(community_id.eq(join_community_id_one.community_id)).order(id).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
                for theme_community_id_one in  theme_community_id {
                    themes_result_user_id.push(theme_community_id_one);
                }
            }
            let mut themes_list_user_id: Vec<ThemeListResult> = vec![];
            for theme_one in themes_result_user_id {
                let mut themes_list_one = ThemeListResult::new();
                themes_list_one.id = theme_one.id;
                themes_list_one.user_id = theme_one.user_id;
                themes_list_one.community_id = theme_one.community_id;
                themes_list_one.theme_status = theme_one.theme_status;
                themes_list_one.title = theme_one.title;
                themes_list_one.content = theme_one.content;
                themes_list_one.view_count = theme_one.view_count;
                themes_list_one.comment_count = theme_one.comment_count;
                themes_list_one.created_at = theme_one.created_at;
                let rtime = time( Utc::now().naive_utc(), theme_one.created_at);
                themes_list_one.rtime = rtime;
                let community_result =  communitys::table.filter(communitys::id.eq(theme_one.community_id)).load::<Community>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match community_result {
                    Some(community_one) => { themes_list_one.community_name = community_one.community_name; },
                    None => { println!("No community result");},
                }
                let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match theme_user {
                    Some(user) => { themes_list_one.username = user.username;},
                    None => { println!("No theme_user result");},
                }
                themes_list_user_id.push(themes_list_one);
            }
            Ok(ThemeListMsgs { 
                status: 200,
                message : "theme_list result.".to_string(),
                theme_list: themes_list_user_id,
            })
        }
    }
}

impl Handler<ThemeId> for ConnDsl {
    type Result = Result<ThemeAndCommentsMsgs, Error>;
    fn handle(&mut self, theme_one: ThemeId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::communitys;
        use utils::schema::comments;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::update(themes).filter(&id.eq(&theme_one.theme_id)).set((view_count.eq(view_count + 1),)).execute(conn).map_err(error::ErrorInternalServerError)?;
        let the_theme =  themes.filter(&id.eq(&theme_one.theme_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let mut theme_and_comments = ThemeAndCommentsMsgs::new();
        match the_theme {
            Some(mut themeid) => {
                themeid.content = markdown_to_html(&themeid.content);
                theme_and_comments.theme = themeid.clone();
                let mut theme_comment = comments::table.filter(&comments::theme_id.eq(&themeid.id)).load::<Comment>(conn).map_err(error::ErrorInternalServerError)?;
                for comment in &mut theme_comment {
                    let mut comment_list_one = CommentReturn::new();
                    comment_list_one.id = comment.id;
                    comment_list_one.theme_id = comment.theme_id;
                    comment_list_one.user_id = comment.user_id;
                    comment_list_one.content = markdown_to_html(&comment.content);
                    comment_list_one.created_at = comment.created_at;
                    let comment_user = users::table.filter(&users::id.eq(comment.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                    match comment_user {
                            Some(someuser) => {  
                            comment_list_one.username = someuser.username; },
                            None => { println!("No comment_user"); },
                    }
                    comment_list_one.rtime = time(Utc::now().naive_utc(), comment.created_at);
                    theme_and_comments.theme_comments.push(comment_list_one);
                }
                let community_result =  communitys::table.filter(&communitys::id.eq(&themeid.community_id)).load::<Community>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match community_result {
                    Some(community_one) => {
                        theme_and_comments.theme_community_name = community_one.community_name;},
                    None => { println!("No theme_community"); },
                }
                theme_and_comments.theme_rtime = time(Utc::now().naive_utc(), themeid.created_at);
                let user_result = users::table.filter(&users::id.eq(&themeid.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match user_result {
                    Some(themeid_user) => { 
                        theme_and_comments.theme_user = themeid_user; },
                    None => { println!("No theme_user"); },
                }
            },
            None => { println!("No theme"); },
        }
        Ok(ThemeAndCommentsMsgs{
            theme: theme_and_comments.theme,
            theme_user: theme_and_comments.theme_user,
            theme_community_name: theme_and_comments.theme_community_name,
            theme_rtime: theme_and_comments.theme_rtime,
            theme_comments: theme_and_comments.theme_comments,
        })
    }
}

impl Handler<ThemeNew> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, theme_new: ThemeNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::communitys;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let community_one =  communitys::table.filter(communitys::community_name.eq(theme_new.community_name)).load::<Community>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let cid: i32 ;
        match community_one {
            Some(one) => { cid = one.id;},
            None => { cid = 0;},
        }    
        let new_theme = NewTheme {
            user_id: theme_new.user_id,
            community_id: cid,
            title: &theme_new.title,
            content: &theme_new.content,
            created_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(themes).values(&new_theme).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                status: 200,
                message : "Theme Post Successful.".to_string(),
        })        
    }
}

impl Handler<ThemeComment> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, theme_comment: ThemeComment, _: &mut Self::Context) -> Self::Result {
        use utils::schema::comments::dsl::*;
        use utils::schema::themes;
        let the_theme_id: i32 = theme_comment.the_theme_id.to_owned().parse().map_err(error::ErrorBadRequest)?;
        let new_comment = NewComment {
            theme_id: the_theme_id,
            user_id: theme_comment.user_id,
            content: &theme_comment.comment,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(comments).values(&new_comment).execute(conn).map_err(error::ErrorInternalServerError)?;
        diesel::update(themes::table)
            .filter(&themes::id.eq(&the_theme_id))
            .set((themes::comment_count.eq(themes::comment_count + 1),))
            .execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                status: 200,
                message : "Comment Add Successful.".to_string(),
        })        
    }
}

