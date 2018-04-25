use diesel;
use diesel::*;
use actix::*;
use actix_web::*;
use chrono::Utc;
use model::response::{ThemeListMsgs, ThemeMsgs, Msgs};
use model::theme::{Theme, ThemeList, ThemeId, NewTheme, ThemeNew, no_theme};
use model::db::ConnDsl;
use model::user::{User, no_user};

impl Handler<ThemeList> for ConnDsl {
    type Result = Result<ThemeListMsgs, Error>;

    fn handle(&mut self, theme_list: ThemeList, _: &mut Self::Context) -> Self::Result {
        use utils::schema::theme::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let themes = sql_query(
            "SELECT theme.id, theme.user_id, theme.category, theme.status, theme.title, theme.content,
                           theme.view_count, theme.comment_count, theme.created_at, users.username 
                           FROM theme, users WHERE theme.user_id = users.id ORDER BY theme.id DESC "
            ).load(conn).map_err(error::ErrorInternalServerError)?;
        Ok(ThemeListMsgs { 
            status: 200,
            message : "theme_list result.".to_string(),
            theme_list: themes,
        })
    }
}

impl Handler<ThemeId> for ConnDsl {
    type Result = Result<ThemeMsgs, Error>;

    fn handle(&mut self, theme_id: ThemeId, _: &mut Self::Context) -> Self::Result {
        use utils::schema::theme::dsl::*;
        use utils::schema::users;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let the_theme =  theme.filter(&id.eq(&theme_id.theme_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
        let no_theme = no_theme();
        let no_user = no_user();
        match the_theme {
            Some(themeid) => {
                let uid = themeid.user_id;
                let user_result = users::table.filter(&users::id.eq(uid)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                match user_result {
                    Some(themeid_user) => {
                        Ok(ThemeMsgs { 
                            status: 200,
                            message : "The  theme info.".to_string(),
                            theme: themeid,
                            theme_user: themeid_user,
                        })
                    },
                    None => {
                        Ok(ThemeMsgs { 
                            status: 400,
                            message : "error.".to_string(),
                            theme: no_theme,
                            theme_user: no_user,
                        })
                    },
                }
                    
            },
            None => {
                    Ok(ThemeMsgs { 
                        status: 400,
                        message : "error.".to_string(),
                        theme: no_theme,
                        theme_user: no_user,
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
