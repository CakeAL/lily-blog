use anyhow::Result;
use entity::entity::post::Column;
use entity::entity::post;
use sea_orm::prelude::{DateTimeWithTimeZone, Expr};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use util::gen_html;

pub async fn insert_new_post(
    db: &DatabaseConnection,
    title: String,
    tag_id: Vec<i32>,
    md_path: String,
    summary: String,
) -> Result<i32> {
    let (html_path, words_len) = gen_html(&md_path)?;
    let new_post = post::ActiveModel {
        title: Set(title),
        tag_id: Set(Some(util::tags_to_u8(tag_id))),
        md_path: Set(md_path),
        html_path: Set(html_path),
        words_len: Set(Some(words_len)),
        summary: Set(summary),
        ..Default::default()
    };
    let res = post::Entity::insert(new_post).exec(db).await?;
    Ok(res.last_insert_id)
}

pub async fn update_post(
    db: &DatabaseConnection,
    id: i32,
    title: String,
    tag_id: Vec<i32>,
    md_path: String,
    summary: String,
) -> Result<u64> {
    let res = post::Entity::update_many()
        .filter(Column::Id.eq(id))
        .col_expr(Column::Title, Expr::value(title))
        .col_expr(Column::TagId, Expr::value(Some(util::tags_to_u8(tag_id))))
        .col_expr(Column::MdPath, Expr::value(md_path))
        .col_expr(Column::Summary, Expr::value(Some(summary)))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

pub async fn select_record_total(
    db: &DatabaseConnection,
    tag_id: Option<i32>,
    keyword: Option<String>,
    is_del: Option<bool>,
    start: Option<DateTimeWithTimeZone>,
    end: Option<DateTimeWithTimeZone>,
) -> Result<u64> {
    todo!()
}