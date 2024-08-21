// proto 类型 到 Rust 结构体类型转换

use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub summary: String,
    // pub md_path: String,
    // pub html_path: String,
    pub hit: i32,
    pub words_len: i32,
    // pub is_del: bool,
    pub publish_time: i64,
    pub update_time: i64,
    pub tag_id: Vec<i32>,
}

impl From<proto::Post> for Post {
    fn from(p: proto::Post) -> Self {
        Self {
            id: p.id,
            title: p.title,
            summary: p.summary,
            // md_path: p.md_path,
            // html_path: p.html_path,
            hit: p.hit,
            words_len: p.words_len,
            // is_del: p.is_del,
            publish_time: p.publish_time.unwrap_or_default().seconds,
            update_time: p.update_time.unwrap_or_default().seconds,
            tag_id: p.tag_id,
        }
    }
}

#[derive(Serialize)]
pub struct ListPostRes {
    pub page: i32,
    pub page_total: i32,
    pub posts: Vec<Post>,
}

#[derive(Serialize)]
pub struct GetPostRes {
    pub post: Post,
    pub content: String,
}

#[derive(Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl From<proto::Tag> for Tag {
    fn from(t: proto::Tag) -> Self {
        Self {
            id: t.id,
            name: t.name,
        }
    }
}

#[derive(Serialize)]
pub struct Comment {
    id: i32,
    post_id: i32,
    name: String,
    hashed_email: String,
    content: String,
    created_at: i64,
}

impl From<proto::Comment> for Comment {
    fn from(c: proto::Comment) -> Self {
        Self {
            id: c.id,
            post_id: c.post_id,
            name: c.name,
            hashed_email: c.hashed_email,
            content: c.content,
            created_at: c.created_at.unwrap_or_default().seconds,
        }
    }
}