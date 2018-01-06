use rocket_contrib::Template;
use std::collections::HashMap;
use rocket::request::Form;
use controller::user::{UserId,UserOr};
use controller::home::{TemplateWiki, TemplateContext, PAGE_SIZE};
use handler::content::{ get_article_by_aid,get_comment_by_aid,add_comment_by_aid,add_wiki_by_wid,get_wiki,get_wiki_by_id};
use handler::content::{ Ucomment,Uarticle,article_list,add_article_by_uid,article_count};
use model::db::ConnDsl;
use model::pg::ConnPg;


#[derive(Debug,Serialize)]
struct TemplateArticle {
    article: Uarticle,
    comments: Vec<Ucomment>,
    username: String,
    user_id: i32,
}

#[derive(FromForm,Debug)]
pub struct DataArticle {
    pub category: String,
    pub title: String,
    pub raw: String,
}

#[derive(FromForm,Debug)]
pub struct DataComment {
    pub aid: Option<i32>,
    pub raw: String,
}

#[get("/<aid>", rank = 2)]
pub fn article_nouser(conn_pg: ConnPg, conn_dsl: ConnDsl, aid: i32) -> Template {
    let article_data = get_article_by_aid(&conn_pg, aid );
    let comment_data = get_comment_by_aid(&conn_pg, aid);
    let context = TemplateArticle {
        article: article_data,
        comments: comment_data,
        username: "".to_string(),
        user_id: 0,
    };
    if context.article.category == "Blog" {
        Template::render("blog", &context)
    }else{
        Template::render("article", &context)
    }
}

#[get("/<aid>")]
pub fn article(conn_pg: ConnPg, conn_dsl: ConnDsl, user: UserOr, aid: i32, user_id: UserId) -> Template {
    let article_data = get_article_by_aid(&conn_pg, aid);
    let comment_data = get_comment_by_aid(&conn_pg, aid);
    let context = TemplateArticle {
        article: article_data,
        comments: comment_data,
        username: user.0,
        user_id: user_id.0,
    };
    if context.article.category == "Blog" {
        Template::render("blog", &context)
    }else{
        Template::render("article", &context)
    }
    
}

#[get("/addcomment?<data_comment>")]
pub fn add_comment(conn_pg: ConnPg, conn_dsl: ConnDsl, user: UserOr, user_id: UserId, data_comment: DataComment)  {
    let uid = user_id.0;
    if let Some(aid) = data_comment.aid {
        let use_aid = aid;
        let use_content = data_comment.raw;
        add_comment_by_aid(&conn_pg, &conn_dsl, use_aid, uid, &use_content);
    } else {
        "Something Wrong!".to_string();
    }
}

#[get("/new")]
pub fn new(conn_dsl: ConnDsl, user: UserOr, user_id: UserId) -> Template {
    let mut context = HashMap::new();
    context.insert("username", user.0);
    context.insert("user_id", user_id.0.to_string());
    Template::render("new", &context)
}

#[post("/addarticle", data = "<data_article>")]
fn add_article(conn_pg: ConnPg, conn_dsl: ConnDsl, user: UserOr, user_id: UserId, data_article: Form<DataArticle>)  -> Template {
    let data = data_article.get();
    let title = &data.title;
    let raw = &data.raw;
    let category = &data.category;
    let wiki = ["Docs", "Resources", "Web", "Embed", "Server", "Client"];
    if wiki.contains(&category.as_str()) {
        add_wiki_by_wid(&conn_dsl, &category, &title, &raw);
        let wikis = get_wiki(&conn_pg);
        let id: i32 = 1;
        let rwiki = get_wiki_by_id(&conn_pg, id);
        let context = TemplateWiki {
            wikis: wikis,
            rwiki: rwiki,
            username: user.0,
            user_id: user_id.0,
        };
        Template::render("wiki", &context)
    }else{
        let uid = user_id.0;
        add_article_by_uid(&conn_dsl, uid, &category, &title, &raw);
        let page: i32 = 1;
        let datas = article_list(&conn_pg,page);
        let article_count = article_count(&conn_pg);
        let mut pcount = (article_count + PAGE_SIZE - 1) / PAGE_SIZE;
        let mut pages: Vec<i32> = vec![];
        while pcount > 0 {
            pages.push(pcount);
            pcount -= 1;
        }
        pages.sort();
        let page_count = pages.len() as i32;
        let previous_page = if page - 1 < 1 {1} else {page - 1};
        let next_page = if page + 1 > page_count {page_count} else {page + 1};
        let context = TemplateContext {
            datas: datas,
            username: user.0,
            user_id: user_id.0,
            count:0,
            pages: pages,
            previous_page: previous_page,
            next_page: next_page,
            tag: "".to_string(),
            page_count: page_count,
        };
        Template::render("index", &context)
    }
    
}
