#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(dotenv_macros)]
#![feature(custom_derive)]
#![feature(custom_attribute)]
#![feature(decl_macro)]
#![recursion_limit = "128"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate postgres;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate dotenv;
extern crate chrono;
extern crate regex;
extern crate spongedown;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_postgres;
extern crate timeago;
extern crate bcrypt;

#[macro_use]
mod controller;
#[macro_use]
mod handler;
#[macro_use]
mod model;
mod utils;

use rocket_contrib::Template;
use controller::{home,user,article};

pub fn start() {
    let pool_dsl = model::db::init_pool();
    let pool_pg = model::pg::init_pool();
    rocket::ignite()
        .manage(pool_dsl)
        .manage(pool_pg)
        .mount("/", routes![home::public,home::index_user_page,home::index_user,home::index_page,home::index,home::index_user_page_tag,home::index_user_tag,home::index_page_tag,home::index_tag,home::wiki_user,home::wiki,home::wiki_id,home::wiki_user_id,home::more_user,home::more])
        .mount("/user",routes![user::login_register,user::register,user::register_post,
                               user::login_user,user::login,user::login_post,user::logout,user::user_page_login_message,user::user_page_login,user::user_page])
        .mount("/article",routes![article::article,article::add_comment,article::article_nouser,article::new,article::add_article])
        .attach(Template::fairing())
        .catch(catchers![home::not_found])
        .launch();
}



