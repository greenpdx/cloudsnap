use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::community::{Communitys, CommunityNew, CommunityNames, CommunityCategorys, CommunityThemes, CommunityLike};

pub fn community_new(
    (community_new, state): (Json<CommunityNew>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state.db.send(CommunityNew {
        create_user_id: community_new.create_user_id,
        community_name: community_new.community_name.clone(),
        community_category: community_new.community_category.clone(),
    })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn community_names(
    (community_names, state): (Json<CommunityNames>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    state.db.send(CommunityNames {
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

pub fn community_categorys(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(CommunityCategorys)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

pub fn communitys(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(Communitys)
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
    req.state().db.send(CommunityThemes {
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

pub fn community_like(
    (community_like, state): (Json<CommunityLike>, State<AppState>),
) -> FutureResponse<HttpResponse> {
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
