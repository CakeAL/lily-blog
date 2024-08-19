// proto 类型 到 Rust 结构体类型转换

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ListPostRes {
    pub page: i32,
    pub page_total: i32,
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPostRes {
    pub post: Post,
    pub content: String,
}