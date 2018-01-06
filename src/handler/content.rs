use diesel;
use diesel::prelude::*;
use model::article::{Article,Comment,NewArticle,NewComment,STATUS};
use model::user::{User,NewMessage,Message,message_mode,message_status};
use model::wiki::{Wiki, NewWiki};
use controller::user::UserId;
use chrono::prelude::*;
use regex::{Regex,Captures};
use chrono::{DateTime,Utc};
use spongedown;
use diesel::pg::PgConnection;
use postgres::Connection;
use std::time::Duration;
use timeago;
use utils::get_seconds::get_seconds;
use controller::home::PAGE_SIZE;

#[derive(Debug, Serialize)]
pub struct Uarticle {
    pub id: i32,
    pub uid: i32,
    pub category: String,
    pub status: i32,
    pub comments_count: i32,
    pub title: String,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub rtime: String,
    pub updated_at: DateTime<Utc>,
    pub username: String,
}
#[derive(Debug, Serialize)]
pub struct Rarticle {
    pub id: i32,
    pub uid: i32,
    pub category: String,
    pub status: i32,
    pub comments_count: i32,
    pub title: String,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub rtime: String,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Serialize)]
pub struct Ucomment {
    pub id: i32,
    pub aid: i32,
    pub uid: i32,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub rtime: String,
    pub username: String,
}
#[derive(Debug,Serialize)]
pub struct UserComment {
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub rtime: String,
    pub article_id: i32,
    pub article_category: String,
    pub article_status: i32,
    pub article_comments_count: i32,
    pub article_title: String,
    pub article_raw: String,
    pub article_cooked: String,
}
#[derive(Debug,Serialize)]
pub struct UserMessage {
    pub message_status: i32,
    pub message_created_at: DateTime<Utc>,
    pub rtime: String,
    pub comment_raw: String,
    pub comment_cooked: String,
    pub from_uid: i32,
    pub from_uid_name: String,
    pub from_uid_email: String,
    pub article_id: i32,
    pub article_title: String,
}
#[derive(Debug,Serialize)]
pub struct Rwiki {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub raw: String,
    pub cooked: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub rtime: String,
}
struct CommentId{
    id: i32,
}
struct ToUid{
    id: i32,
}
struct UnreadStatus{
    status: i32,
}
pub fn article_count(conn_pg: &Connection) -> i32 {
    struct Aid {
        id: i32,
    }
    let mut counts: Vec<Aid> = vec![];
    for row in &conn_pg.query("SELECT article.id FROM article ", &[]).unwrap()
    {
        let a_id = Aid {
            id: row.get(0),
        };
        counts.push(a_id);
    }
    counts.len() as i32
}
pub fn article_count_tag<'a>(conn_pg: &Connection, tag: &'a str) -> i32 {
    struct Aid {
        id: i32,
    }
    let mut counts: Vec<Aid> = vec![];
    for row in &conn_pg.query("SELECT article.id FROM article WHERE article.category = $1", &[&tag]).unwrap()
    {
        let a_id = Aid {
            id: row.get(0),
        };
        counts.push(a_id);
    }
    counts.len() as i32
}
pub fn article_count_no_comment(conn_pg: &Connection) -> i32 {
    struct Aid {
        id: i32,
    }
    let mut counts: Vec<Aid> = vec![];
    for row in &conn_pg.query("SELECT article.id FROM article WHERE article.comments_count = 0", &[]).unwrap()
    {
        let a_id = Aid {
            id: row.get(0),
        };
        counts.push(a_id);
    }
    counts.len() as i32
}
pub fn article_list_no_comment(conn_pg: &Connection, page: i32) -> Vec<Uarticle> {
    let page = page as i64;
    let p_size = PAGE_SIZE as i64;
    let mut article_result: Vec<Uarticle> = vec![];
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, article.comments_count, article.title, article.raw,
                           article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND article.category = 'Announcement' ORDER BY article.id DESC ", &[]).unwrap()
    {
        let mut result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
            username: row.get(10),
        };
        let created_at_seconds = get_seconds(result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        result.rtime = dtime;
        article_result.push(result);
    }
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, article.comments_count, article.title, article.raw,
                           article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND article.comments_count = 0 AND article.category != 'Announcement' ORDER BY article.id DESC limit $1 OFFSET $2", &[&p_size, &((page - 1) * p_size)]).unwrap()
    {
        let mut result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
            username: row.get(10),
        };
        let created_at_seconds = get_seconds(result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        result.rtime = dtime;
        article_result.push(result);
    }
    article_result
}
pub fn article_list(conn_pg: &Connection, page: i32) -> Vec<Uarticle> {
    let page = page as i64;
    let p_size = PAGE_SIZE as i64;
    let mut article_result: Vec<Uarticle> = vec![];
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, article.comments_count, article.title, article.raw,
                           article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND article.category = 'Announcement' ORDER BY article.id DESC ", &[]).unwrap()
    {
        let mut result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
            username: row.get(10),
        };
        let created_at_seconds = get_seconds(result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        result.rtime = dtime;
        article_result.push(result);
    }
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, article.comments_count, article.title, article.raw,
                           article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND article.category != 'Announcement' ORDER BY article.id DESC limit $1 OFFSET $2", &[&p_size, &((page - 1) * p_size)]).unwrap()
    {
        let mut result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
            username: row.get(10),
        };
        let created_at_seconds = get_seconds(result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        result.rtime = dtime;
        article_result.push(result);
    }
    article_result
}
pub fn article_list_tag<'a>(conn_pg: &Connection, page: i32, tag: &'a str) -> Vec<Uarticle> {
    let page = page as i64;
    let p_size = PAGE_SIZE as i64;
    let mut article_result: Vec<Uarticle> = vec![];
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, article.comments_count, article.title, article.raw,
                           article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND article.category = 'Announcement' ORDER BY article.id DESC ", &[]).unwrap()
    {
        let mut result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
            username: row.get(10),
        };
        let created_at_seconds = get_seconds(result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        result.rtime = dtime;
        article_result.push(result);
    }
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, article.comments_count, article.title, article.raw,
                           article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND  article.category = $1 
                           ORDER BY article.id DESC limit $2 OFFSET $3", &[&tag, &p_size, &((page - 1) * p_size)]).unwrap()
    {
        let mut result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
            username: row.get(10),
        };
        let created_at_seconds = get_seconds(result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        result.rtime = dtime;
        article_result.push(result);
    }
    article_result
}
pub fn get_article_by_aid(conn_pg: &Connection, aid: i32) -> Uarticle {
    let mut article_result = Uarticle {
        id: 0,
        uid: 0,
        category: "".to_string(),
        status: 0,
        comments_count: 0,
        title: "".to_string(),
        raw: "".to_string(),
        cooked: "".to_string(),
        created_at: Utc::now(), 
        rtime: "".to_string(),
        updated_at: Utc::now(), 
        username: "".to_string(),
    };
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status,
                            article.comments_count, article.title, article.raw, article.cooked, article.created_at, article.updated_at, users.username 
                           FROM article, users WHERE article.uid = users.id AND article.id = $1", &[&aid]).unwrap() {
        article_result = Uarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
            username: row.get(10),
        };
        let created_at_seconds = get_seconds(article_result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        article_result.rtime = dtime;
    }
    article_result
}

pub fn get_comment_by_aid(conn_pg: &Connection, aid: i32) -> Vec<Ucomment> {
    let mut result: Vec<Ucomment> = vec![];
    for row in &conn_pg.query("SELECT comment.id, comment.aid, comment.uid, comment.raw, comment.cooked, comment.created_at, users.username 
                            FROM comment, users WHERE comment.uid = users.id AND comment.aid = $1 ORDER BY comment.id", &[&aid]).unwrap() {
        let mut comment_result = Ucomment {
            id: row.get(0),
            aid: row.get(1),
            uid: row.get(2),
            raw: row.get(3),
            cooked: row.get(4),
            created_at: row.get(5),
            rtime: "".to_string(),
            username: row.get(6),
        };
        let created_at_seconds = get_seconds(comment_result.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        comment_result.rtime = dtime;
        result.push(comment_result);
    }
    result
}

pub fn add_article_by_uid<'a>(conn_dsl: &PgConnection, uid: i32, category: &'a str, title: &'a str, raw: &'a str) {
    use utils::schema::article;
    let created_at = Utc::now();
    let updated_at = Utc::now();
    let new_article = NewArticle {
        uid: uid,
        category: category,
        status:STATUS::NORMAL,
        comments_count:0,
        title: title,
        raw: raw,
        cooked: &spongedown::parse(&raw),
        created_at: created_at,
        updated_at: updated_at,
    };
    diesel::insert(&new_article).into(article::table).execute(conn_dsl).expect("Error saving new list");
}

pub fn add_wiki_by_wid<'a>(conn_dsl: &PgConnection, category: &'a str, title: &'a str, raw: &'a str) {
    use utils::schema::wiki;
    let created_at = Utc::now();
    let updated_at = Utc::now();
    let new_wiki = NewWiki {
        category: category,
        title: title,
        raw: raw,
        cooked: &spongedown::parse(&raw),
        created_at: created_at,
        updated_at: updated_at,
    };
    diesel::insert(&new_wiki).into(wiki::table).execute(conn_dsl).expect("Error saving new wiki");
}
        
pub fn add_comment_by_aid<'a>(conn_pg: &Connection, conn_dsl: &PgConnection, aid: i32, uid: i32, raw: &'a str) {
   
    use utils::schema::comment;
    let re = Regex::new(r"\B@([\da-zA-Z_]+)").unwrap();
    let mut to_uids: Vec<i32> = Vec::new();
    let new_content = re.replace_all(&raw, |cap: &Captures| {
        match get_uids(conn_pg, cap.at(1).unwrap()) {
            Some(user_id) => {
                to_uids.push(user_id);
                format!("[@{}]({}{})",
                        cap.at(1).unwrap(),
                        "/user/",
                        user_id)
            },
            None => format!("@{}", cap.at(1).unwrap()),
        }
    });
    let created_at = Utc::now();
    let new_comment = NewComment {
        aid : aid,
        uid : uid,
        raw : &new_content,
        cooked: &spongedown::parse(&new_content),
        created_at : created_at,
    };
    diesel::insert(&new_comment).into(comment::table).execute(conn_dsl).expect("Error saving new comment");
    
    let mut comment_id: i32 = 0;
    for row in &conn_pg.query("SELECT comment.id FROM comment WHERE comment.raw = $1",&[&raw]).unwrap() {
        let comment = CommentId {
            id: row.get(0),
        };
        comment_id = comment.id;
    }
    conn_pg.execute("UPDATE article SET comments_count = comments_count + 1 WHERE id = $1", &[&aid]).unwrap();
    let mut author_id: i32 = 0;
    for row in &conn_pg.query("SELECT article.uid FROM article WHERE article.id = $1",&[&aid]).unwrap() {
        let t_uid = ToUid {
            id: row.get(0),
        };
        author_id = t_uid.id;
    }
    let cooked = &spongedown::parse(&new_content);
    //send message to article author.
    if uid != author_id {
        conn_pg.execute("INSERT INTO message (aid, cid, from_uid, to_uid, raw, cooked, mode, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8,$9)",
                 &[&aid, &comment_id, &uid, &author_id, &raw, &cooked, &message_mode::REPLY_ARTICLE, &message_status::INIT, &created_at]).unwrap();
    }
    //send message to mentions.
    to_uids.sort();
    to_uids.dedup();
    for to_uid in to_uids.iter().filter(|&x| *x != author_id && *x != uid) {
        conn_pg.execute("INSERT INTO message(aid, cid, from_uid, to_uid, raw, cooked, mode, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8,$9)",
                &[&aid, &comment_id, &uid, &to_uid, &raw, &cooked, &message_mode::MENTION, &message_status::INIT, &created_at]).unwrap();
    }
}

pub fn get_uids(conn_pg: &Connection, username: &str) -> Option<i32> {

    let mut to_uid: Option<i32> = Some(0);
    for row in &conn_pg.query("SELECT id FROM users WHERE username = $1",&[&username]).unwrap() {
        let uid = ToUid {
            id: row.get(0),
        };
        to_uid = Some(uid.id);
    }
    to_uid
}

pub fn get_user_info(conn_dsl: &PgConnection, u_id: i32) -> Option<User> {
    use utils::schema::users::dsl::*;
    let user_result =  users.filter(&id.eq(&u_id)).load::<User>(conn_dsl);
    let login_user = match user_result {
        Ok(user_s) => match user_s.first() {
            Some(a_user) => Some(a_user.clone()),
            None => None,
        },
        Err(_) => None,
    };
    login_user
}

pub fn get_user_articles(conn_pg: &Connection, u_id: i32) -> Vec<Rarticle> {
    let mut user_articles: Vec<Rarticle> = vec![];
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, 
                            article.comments_count, article.title, article.raw, article.cooked, article.created_at, article.updated_at 
                           FROM article WHERE article.uid = $1 ORDER BY created_at DESC",&[&u_id]).unwrap() {
        let mut rarticle = Rarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
        };
        let created_at_seconds = get_seconds(rarticle.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        rarticle.rtime = dtime;
        user_articles.push(rarticle);
    }
    user_articles
}
pub fn get_user_blogs(conn_pg: &Connection, u_id: i32) -> Vec<Rarticle> {
    let mut user_blogs: Vec<Rarticle> = vec![];
    for row in &conn_pg.query("SELECT article.id, article.uid, article.category, article.status, 
                            article.comments_count, article.title, article.raw, article.cooked, article.created_at, article.updated_at 
                           FROM article WHERE article.uid = $1 ORDER BY created_at DESC",&[&u_id]).unwrap() {
        let mut rarticle = Rarticle {
            id: row.get(0),
            uid: row.get(1),
            category: row.get(2),
            status: row.get(3),
            comments_count: row.get(4),
            title: row.get(5),
            raw: row.get(6),
            cooked: row.get(7),
            created_at: row.get(8),
            rtime: "".to_string(),
            updated_at: row.get(9),
        };
        let created_at_seconds = get_seconds(rarticle.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        rarticle.rtime = dtime;
        user_blogs.push(rarticle);
    }
    user_blogs
}
pub fn get_user_comments(conn_pg: &Connection, u_id: i32) -> Vec<UserComment> {
    let mut user_comments: Vec<UserComment> = vec![];
    for row in &conn_pg.query("SELECT comment.raw, comment.cooked, comment.created_at, 
                               article.id, article.category, article.status, article.comments_count, article.title, article.raw, article.cooked
                               FROM comment, article 
                        where comment.aid = article.id and comment.uid = $1 order by comment.id DESC ",&[&u_id]).unwrap() {
        let mut comment = UserComment {
                raw: row.get(0),
                cooked: row.get(1),
                created_at: row.get(2),
                rtime: "".to_string(),
                article_id: row.get(3),
                article_category: row.get(4),
                article_status: row.get(5),
                article_comments_count: row.get(6),
                article_title: row.get(7),
                article_raw: row.get(8),
                article_cooked: row.get(9),
        };
        let created_at_seconds = get_seconds(comment.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        comment.rtime = dtime;
        user_comments.push(comment);
    }
    user_comments
}

pub fn get_user_messages(conn_pg: &Connection, u_id: i32) -> Vec<UserMessage> {
    let mut user_messages: Vec<UserMessage> = vec![];
    for row in &conn_pg.query("SELECT message.status, message.created_at, comment.raw, comment.cooked, users.id, users.username,
         users.email, article.id, article.title 
         FROM message
         JOIN users ON message.from_uid = users.id
         JOIN article ON article.id = message.aid
         JOIN comment ON comment.id = message.cid
         WHERE to_uid = $1 ORDER BY created_at DESC",&[&u_id]).unwrap() {
        let mut message = UserMessage {
                message_status: row.get(0),
                message_created_at: row.get(1),
                rtime: "".to_string(),
                comment_raw: row.get(2),
                comment_cooked: row.get(3),
                from_uid: row.get(4),
                from_uid_name: row.get(5),
                from_uid_email: row.get(6),
                article_id: row.get(7),
                article_title: row.get(8),
        };
        let created_at_seconds = get_seconds(message.message_created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        message.rtime = dtime;
        user_messages.push(message);
    }
    user_messages
}

pub fn get_unread_message_count(conn_pg: &Connection, to_uid: i32) -> i32 {
    let mut all_count: Vec<UnreadStatus> = vec![];
    let mut read_count: i32 = 0;
    for row in &conn_pg.query("SELECT message.status from message where to_uid = $1 and status = $2", &[&to_uid,&message_status::INIT]).unwrap() {
        let message = UnreadStatus {
                status: row.get(0),
        };
        read_count += message.status;
        all_count.push(message);
    }
    let all_count = all_count.len() as i32 ;
    all_count - read_count

}

pub fn update_user_message(conn_pg: &Connection, to_uid: i32, count: i32) {
    for i in 0..count {
        &conn_pg.execute("UPDATE message SET status = $1 WHERE to_uid = $2", &[&message_status::READ, &to_uid]).unwrap();
    }
    
}

pub fn get_wiki(conn_pg: &Connection) -> Vec<Rwiki> {
    let mut wiki_result: Vec<Rwiki> = vec![];
    for row in &conn_pg.query("SELECT wiki.id, wiki.category, wiki.title, wiki.raw, wiki.cooked, wiki.created_at, wiki.updated_at 
                           FROM wiki ", &[]).unwrap()
    {
        let mut wiki = Rwiki {
            id: row.get(0),
            category: row.get(1),
            title: row.get(2),
            raw: row.get(3),
            cooked: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
            rtime: "".to_string(),
        };
        let created_at_seconds = get_seconds(wiki.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        wiki.rtime = dtime;
        wiki_result.push(wiki);
    }
    wiki_result
}

pub fn get_wiki_by_id(conn_pg: &Connection, id: i32) -> Rwiki {
    let mut wiki = Rwiki {
        id: 0,
        category: "".to_string(),
        title: "".to_string(),
        raw: "".to_string(),
        cooked: "".to_string(),
        created_at: Utc::now(), 
        updated_at: Utc::now(), 
        rtime: "".to_string(),    
    };
    for row in &conn_pg.query("SELECT wiki.id, wiki.category, wiki.title, wiki.raw, wiki.cooked, wiki.created_at, wiki.updated_at 
                           FROM wiki WHERE wiki.id = $1", &[&id]).unwrap()
    {
        wiki = Rwiki {
            id: row.get(0),
            category: row.get(1),
            title: row.get(2),
            raw: row.get(3),
            cooked: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
            rtime: "".to_string(),
        };
        let created_at_seconds = get_seconds(wiki.created_at);
        let rnow = Utc::now();
        let rnow_seconds = get_seconds(rnow);
        let ntime = rnow_seconds - created_at_seconds;
        let dtime = timeago::format(Duration::new(ntime, 0), timeago::Style::LONG);
        wiki.rtime = dtime;
    }
    wiki
}