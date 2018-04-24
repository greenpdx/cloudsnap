use diesel;
use diesel::*;
use actix::*;
use actix_web::*;
use chrono::Utc;
use model::response::{ThemeListMsgs, ThemeMsgs, Msgs};
use model::theme::{Theme, ThemeList, ThemeId, NewTheme, ThemeNew};
use model::db::ConnDsl;

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
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let the_theme =  theme.filter(&id.eq(&theme_id.theme_id)).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match the_theme {
            Some(the_theme) => {
                    let current_theme = Theme {
                        id: the_theme.id,
                        user_id: the_theme.user_id,
                        category: the_theme.category.clone(),
                        status: the_theme.status,
                        title: the_theme.title.clone(),
                        content: the_theme.content.clone(),
                        view_count: the_theme.view_count,
                        comment_count: the_theme.comment_count,
                        created_at: the_theme.created_at.clone(),
                    };
                    Ok(ThemeMsgs { 
                        status: 200,
                        message : "The  current_user info.".to_string(),
                        theme: current_theme,
                    })
            },
            None => {
                    let no_theme = Theme {
                        id: 0,
                        user_id: 0,
                        category: "".to_owned(),
                        status: 0,
                        title: "".to_owned(),
                        content: "".to_owned(),
                        view_count: 0,
                        comment_count: 0,
                        created_at: Utc::now().naive_utc(),
                    };
                    Ok(ThemeMsgs { 
                        status: 400,
                        message : "error.".to_string(),
                        theme: no_theme,
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
