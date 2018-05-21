use diesel;
use diesel::prelude::*;
use actix::*;
use actix_web::*;
use chrono::Utc;
use std::collections::HashSet;
use model::response::{CommunityNamesMsgs, CommunityCategorysMsgs, CommunitysMsgs, Msgs, CommunityThemesMsgs};
use model::db::ConnDsl;
use model::theme::Theme;
use model::user::User;
use model::join::NewJoin;
use utils::time;
use model::community::{Community, Communitys, NewCommunity, CommunityLike, CommunityNew, CommunityNames, CommunityCategorys, CommunityThemes, CommunityThemeListResult};


impl Handler<CommunityNew> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, community_new: CommunityNew, _: &mut Self::Context) -> Self::Result {
        use utils::schema::communitys::dsl::*;

        let new_community = NewCommunity {
            create_user_id: community_new.create_user_id,
            community_name: &community_new.community_name,
            community_category: &community_new.community_category,
            created_at: Utc::now().naive_utc(),
        };
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        diesel::insert_into(communitys).values(&new_community).execute(conn).map_err(error::ErrorInternalServerError)?;
        Ok(Msgs { 
                status: 200,
                message : "Article Publish Successful.".to_string(),
        })        
    }
}

impl Handler<CommunityNames> for ConnDsl {
    type Result = Result<CommunityNamesMsgs, Error>;

    fn handle(&mut self, community_names: CommunityNames, _: &mut Self::Context) -> Self::Result {
        use utils::schema::communitys::dsl::*;
        let mut name_list: Vec<String> = vec![];
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let community_list = communitys.load::<Community>(conn).map_err(error::ErrorInternalServerError)?;
        for community_one in community_list {
            name_list.push(community_one.community_name);
        }
        Ok(CommunityNamesMsgs { 
                status: 200,
                message : "TypeNamesMsgs.".to_string(),
                community_names: name_list,
        })        
    }
}

impl Handler<CommunityCategorys> for ConnDsl {
    type Result = Result<CommunityCategorysMsgs, Error>;

    fn handle(&mut self, community_categorys: CommunityCategorys, _: &mut Self::Context) -> Self::Result {
        use utils::schema::communitys::dsl::*;
        let mut categorys_list: Vec<String> = vec![];
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let community_list = communitys.load::<Community>(conn).map_err(error::ErrorInternalServerError)?;
        for community_one in community_list {
            categorys_list.push(community_one.community_category);
        }
        let categorys_result = categorys_list.into_iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
        Ok(CommunityCategorysMsgs { 
                status: 200,
                message : "TypeNamesMsgs.".to_string(),
                community_categorys: categorys_result,
        })        
    }
}

impl Handler<Communitys> for ConnDsl {
    type Result = Result<CommunitysMsgs, Error>;

    fn handle(&mut self, communitys: Communitys, _: &mut Self::Context) -> Self::Result {
        use utils::schema::communitys::dsl::*;
        let mut categorys_list: Vec<String> = vec![];
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let community_list = communitys.load::<Community>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(CommunitysMsgs { 
                status: 200,
                message : "TypeNamesMsgs.".to_string(),
                communitys: community_list,
        })        
    }
}

impl Handler<CommunityThemes> for ConnDsl {
    type Result = Result<CommunityThemesMsgs, Error>;

    fn handle(&mut self, community_themes: CommunityThemes, _: &mut Self::Context) -> Self::Result {
        use utils::schema::themes::dsl::*;
        use utils::schema::users;
        use utils::schema::communitys;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let mut themes_result = themes.order(id).load::<Theme>(conn).map_err(error::ErrorInternalServerError)?;
        let mut community_themes_list: Vec<CommunityThemeListResult> = vec![];
        'community_theme_names: for theme_one in themes_result {
            let mut community_themes_list_one = CommunityThemeListResult::new();
            community_themes_list_one.id = theme_one.id;
            community_themes_list_one.user_id = theme_one.user_id;
            community_themes_list_one.community_id = theme_one.community_id;
            community_themes_list_one.theme_status = theme_one.theme_status;
            community_themes_list_one.title = theme_one.title;
            community_themes_list_one.content = theme_one.content;
            community_themes_list_one.view_count = theme_one.view_count;
            community_themes_list_one.comment_count = theme_one.comment_count;
            community_themes_list_one.created_at = theme_one.created_at;
            community_themes_list_one.rtime = time( Utc::now().naive_utc(), theme_one.created_at);
            let community_result =  communitys::table.filter(communitys::id.eq(theme_one.community_id)).load::<Community>(conn).map_err(error::ErrorInternalServerError)?.pop();
            match community_result {
                Some(community_one) => {
                    if community_themes.community_name == community_one.community_name {
                            let theme_user =  users::table.filter(users::id.eq(theme_one.user_id)).load::<User>(conn).map_err(error::ErrorInternalServerError)?.pop();
                            match theme_user {
                                Some(user) => {
                                        community_themes_list_one.username = user.username;
                                        community_themes_list.push(community_themes_list_one);
                                },
                                None => { println!("No user result"); },
                            }
                    }else{ continue 'community_theme_names ;}
                },
                None => { println!("No community result"); },
            }
        }
        Ok(CommunityThemesMsgs { 
            status: 200,
            message : "theme_list result.".to_string(),
            community_theme_list: community_themes_list,
        })       
    }
}

impl Handler<CommunityLike> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, community_like: CommunityLike, _: &mut Self::Context) -> Self::Result {
        use utils::schema::joins::dsl::*;
        use utils::schema::communitys;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let community_result = communitys::table.filter(communitys::community_name.eq(community_like.community_name)).load::<Community>(conn).map_err(error::ErrorInternalServerError)?.pop();
        match community_result {
            Some(community_one) => {
                let create_user_id = community_one.create_user_id;
                if community_like.user_id == create_user_id  {
                    let new_join = NewJoin {
                        user_id: community_like.user_id,
                        user_role: "admin",
                        community_id: community_one.id,
                        created_at: Utc::now().naive_utc(),
                    };
                    diesel::insert_into(joins).values(&new_join).execute(conn).map_err(error::ErrorInternalServerError)?;
                }else{
                    let new_join = NewJoin {
                        user_id: community_like.user_id,
                        user_role: "common",
                        community_id: community_one.id,
                        created_at: Utc::now().naive_utc(),
                    };
                    diesel::insert_into(joins).values(&new_join).execute(conn).map_err(error::ErrorInternalServerError)?;
                }
            },
            None => { println!("No Community result");},
        }
        Ok(Msgs { 
            status: 200,
            message : "community like successful.".to_string(),
        })       
    }
}