use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::community::{CommunityNew, CommunityNames, CommunityThemes, CommunityLike};

pub fn community_new(community_new: Json<CommunityNew>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(CommunityNew{ 
            create_user_id: community_new.create_user_id,
            community_name: community_new.community_name.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn community_names(community_names: Json<CommunityNames>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(CommunityNames{ 
            create_user_id: community_names.create_user_id,
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn community_theme_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let header_community_name = req.match_info().get("community_name").unwrap();
    let community_name = header_community_name.to_string();
    req.state().db.send(CommunityThemes{
        community_name: community_name,
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}

pub fn community_like(community_like: Json<CommunityLike>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(CommunityLike {
        user_id: community_like.user_id,
        community_name: community_like.community_name.clone(),
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}
