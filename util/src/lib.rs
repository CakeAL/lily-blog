pub mod password;

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
    BlogApi,
}

pub async fn get_db_connection() -> Result<DatabaseConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    Ok(db)
}

pub fn get_service_addr(srv: Service) -> Result<String> {
    dotenv().ok();
    let value = match srv {
        Service::Tag => env::var("TAG_SRV_PORT")?,
        Service::Post => env::var("POST_SRV_PORT")?,
        Service::Comment => env::var("COMMENT_SRV_PORT")?,
        Service::Admin => env::var("ADMIN_SRV_PORT")?,
        Service::BlogApi => env::var("BLOG_API_PORT")?,
    };
    Ok(format!("[::1]:{}", value))
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
    // println!("{cleaned_md}");
    Ok((
        path.to_str()
            .ok_or(anyhow!("Path to_str() err!"))?
            .to_string(),
        words_len,
    ))
}

/// get_summary 获取 md_path 路径文章的前 200 字
pub fn get_summary(md_path: &str) -> Result<String> {
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
    tm.map(|tm| DateTimeWithTimeZone::from(Local.timestamp_opt(tm.seconds, 0).unwrap()))
}

pub fn datetime_conversion(dt: Option<DateTimeWithTimeZone>) -> Option<prost_types::Timestamp> {
    match dt {
        Some(dt) => {
            let timestamp = prost_types::Timestamp {
                seconds: dt.timestamp(),
                nanos: 0,
            };
            Some(timestamp)
        }
        None => None,
    }
}

pub fn i64_to_dateline_range(i: Option<(i64, i64)>) -> Option<proto::DatelineRange> {
    match i {
        Some(i) => {
            let start = prost_types::Timestamp {
                seconds: i.0,
                nanos: 0,
            };
            let end = prost_types::Timestamp {
                seconds: i.0,
                nanos: 0,
            };
            Some(proto::DatelineRange {
                start: Some(start),
                end: Some(end),
            })
        }
        None => None,
    }
}

pub fn tags_to_u8(tags: Vec<i32>) -> Vec<u8> {
    // 数据库中使用 X1X 进行间隔存储，比如 X1XX2X 进行查询时直接查询 X1X 即可
    // （防止只使用一个间隔符号，例如 11X12X 查询 1X 出错）
    use std::fmt::Write;
    tags.iter()
        .fold(String::new(), |mut str, num| {
            let _ = write!(str, "X{num}X");
            str
        })
        .into_bytes()
}

pub fn u8_to_tags(bytes: Vec<u8>) -> Vec<i32> {
    // 将 Vec<u8> 转换为 String
    let string = String::from_utf8(bytes).unwrap_or("".to_string()); // 直接忽略算了

    // 分割字符串并过滤空字符串
    string
        .split('X')
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{gen_html, get_summary, tags_to_u8, u8_to_tags};

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

    #[test]
    fn test_u8_to_tags() {
        let tags = vec![1, 2, 3, 4, 5];
        let bytes = tags_to_u8(tags.clone());
        dbg!(String::from_utf8(bytes.clone()).unwrap());
        let reformed_tags = u8_to_tags(bytes);
        assert_eq!(tags, reformed_tags);
    }
}
