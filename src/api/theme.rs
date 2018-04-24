use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::theme::{ThemeList, ThemeId, ThemeNew};


pub fn theme(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let header_theme_id = req.match_info().get("theme_id").unwrap();
    let theme_id: i32 = header_theme_id.parse().unwrap();
    req.state().db.send(ThemeId{theme_id})
       .from_err()
       .and_then(|res| {
           match res {
               Ok(theme) =>
                   Ok(HttpResponse::Ok().json(theme)),
               Err(_) =>
                   Ok(HttpResponse::InternalServerError().into()),
           }
       }).responder()
}

pub fn theme_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(ThemeList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(theme_list) =>
                    Ok(HttpResponse::Ok().json(theme_list)),
                Err(_) =>
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}

pub fn theme_new(theme_new: Json<ThemeNew>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(ThemeNew{ 
            user_id: theme_new.user_id.clone(),
            category: theme_new.category.clone(),
            title: theme_new.title.clone(),
            content: theme_new.content.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

