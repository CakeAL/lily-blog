use anyhow::{anyhow, Result};
use dotenv::dotenv;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sqlx::types::chrono::{Local, TimeZone};
use sea_orm::{Database, DatabaseConnection};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, fs};

pub enum Service {
    Tag,
    Post,
    Comment,
    Admin,
}

pub async fn get_db_connection() -> Result<DatabaseConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    Ok(db)
}

pub fn get_service_addr(srv: Service) -> Result<String> {
    dotenv().ok();
    match srv {
        Service::Tag => Ok(format!("[::1]:{}", env::var("TAG_SRV_PORT")?)),
        Service::Post => Ok(format!("[::1]:{}", env::var("POST_SRV_PORT")?)),
        Service::Comment => Ok(format!("[::1]:{}", env::var("COMMENT_SRV_PORT")?)),
        Service::Admin => Ok(format!("[::1]:{}", env::var("ADMIN_SRV_PORT")?)),
    }
}

pub fn get_service_url(srv: Service) -> Result<String> {
    Ok(format!("http://{}", get_service_addr(srv)?))
}

/// gen_html 从 markdown 文件生成 HTML 放入 gen_html 文件夹中，返回路径
pub fn gen_html(md_path: &str) -> Result<(String, i32)> {
    let file_name = Path::new(md_path)
        .file_stem()
        .ok_or(anyhow!("No file name!"))?
        .to_str()
        .ok_or(anyhow!("File name failed from OsStr to str"))?;
    let md = fs::read_to_string(md_path)?;
    let html = markdown::to_html(&md);

    // 存放路径
    let mut path = env::current_dir()?;
    path.push("gen_html");
    // 检查是否存在这个文件夹
    if path.read_dir().is_err() {
        fs::create_dir(path.clone())?;
    }
    path.push(format!("{file_name}.html"));

    fs::write(path.clone(), html)?;
    let cleaned_md = clean_markdown(&md);
    let words_len = cleaned_md.chars().count() as i32;
    println!("{cleaned_md}");
    Ok((
        path.to_str()
            .ok_or(anyhow!("Path to_str() err!"))?
            .to_string(),
        words_len,
    ))
}

/// get_summary 获取 md_path 路径文章的前 200 字
pub fn get_summary(md_path: &str) -> anyhow::Result<String> {
    let file = File::open(md_path)?;
    let reader = BufReader::new(file);
    let mut res = String::new();
    for line in reader.lines() {
        let line = line?;
        let cleaned_line = clean_markdown(&line);
        if res.len() >= 200 {
            break;
        } else if res.len() + cleaned_line.len() > 200 {
            let additional_chars_needed = 200 - res.len();
            res.push_str(&cleaned_line[..additional_chars_needed]);
            break;
        } else {
            res.push_str(&cleaned_line);
        }
        res.push(' '); // 每行结尾放个空格
    }

    Ok(res)
}

fn clean_markdown(text: &str) -> String {
    text.chars()
        .filter(|c| {
            // 去除Markdown符号，这里只是一个简单的例子，可能需要根据实际情况调整
            !matches!(
                c,
                '*' | '#'
                    | '_'
                    | '>'
                    | '['
                    | ']'
                    | '('
                    | ')'
                    | '`'
                    | '!'
                    | '&'
                    | '|'
                    | '-'
                    | '+'
                    | '='
                    | ' '
                    | '\n'
                    | '\r'
            )
        })
        .collect()
}

pub fn timestamp_conversion(tm: Option<prost_types::Timestamp>) -> Option<DateTimeWithTimeZone> {
    match tm {
        Some(tm) => Some(DateTimeWithTimeZone::from(
            Local.timestamp_opt(tm.seconds, 0).unwrap(),
        )),
        None => None,
    }
}

pub fn tags_to_u8(tags: Vec<i32>) -> Vec<u8> {
    tags.iter()
        .map(|num| format!("'{}'", num))
        .collect::<String>()
        .into_bytes()
}

#[cfg(test)]
mod tests {
    use crate::{gen_html, get_summary};

    #[test]
    fn test_gen_html() {
        let md_path = "/Users/cakeal/Desktop/vsc/lily-blog/README.md";
        let res = gen_html(md_path);
        dbg!(res.unwrap());
    }

    #[test]
    fn test_get_summary() {
        let md_path = "/Users/cakeal/Desktop/vsc/lily-blog/README.md";
        let res = get_summary(md_path);
        dbg!(res.unwrap());
    }
}
