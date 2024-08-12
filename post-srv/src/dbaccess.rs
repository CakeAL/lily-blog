use anyhow::{anyhow, Result};
use entity::entity::post;
use entity::entity::post::Column;
use sea_orm::prelude::{DateTimeWithTimeZone, Expr};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use sea_orm::sqlx::types::chrono::Local;
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
        .col_expr(Column::UpdateTime, Expr::value(Some(DateTimeWithTimeZone::from(Local::now()))))
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
    let mut select = post::Entity::find();
    if let Some(tag_id) = tag_id {
        // 这就是一坨 💩
        select = select.filter(Expr::cust(format!(
            "convert_from(tag_id, 'UTF8') LIKE '%X{}X%'",
            tag_id
        )));
    }
    if let Some(keyword) = keyword {
        select = select.filter(Column::Title.contains(keyword));
    }
    if let Some(is_del) = is_del {
        select = select.filter(Column::IsDel.eq(is_del));
    }
    if start.is_some() && end.is_some() {
        select = select.filter(Column::PublishTime.between(start, end));
    }

    Ok(select.count(db).await?)
}

#[allow(clippy::too_many_arguments)]
pub async fn select_posts(
    db: &DatabaseConnection,
    tag_id: Option<i32>,
    keyword: Option<String>,
    is_del: Option<bool>,
    start: Option<DateTimeWithTimeZone>,
    end: Option<DateTimeWithTimeZone>,
    page_size: i32,
    offset: i32,
) -> Result<Vec<post::Model>> {
    let mut select = post::Entity::find();
    if let Some(tag_id) = tag_id {
        // 这就是一坨 💩
        select = select.filter(Expr::cust(format!(
            "convert_from(tag_id, 'UTF8') LIKE '%X{}X%'",
            tag_id
        )));
    }
    if let Some(keyword) = keyword {
        select = select.filter(Column::Title.contains(keyword));
    }
    if let Some(is_del) = is_del {
        select = select.filter(Column::IsDel.eq(is_del));
    }
    if start.is_some() && end.is_some() {
        select = select.filter(Column::PublishTime.between(start, end));
    }
    let res = select
        .order_by_desc(Column::Id)
        .limit(Some(page_size as u64))
        .offset(Some(offset as u64))
        .all(db)
        .await?;
    Ok(res)
}

pub async fn update_post_del(db: &DatabaseConnection, id: i32) -> Result<bool> {
    let res = post::Entity::update_many()
        .filter(Column::Id.eq(id))
        .col_expr(Column::IsDel, Expr::col(Column::IsDel).not())
        .exec_with_returning(db)
        .await?;

    match res.first() {
        Some(row) => Ok(row.is_del),
        None => Err(anyhow!("No such tag")),
    }
}

pub async fn select_a_post(
    db: &DatabaseConnection,
    id: i32,
    is_del: Option<bool>,
    inc_hit: Option<bool>,
) -> Result<Option<post::Model>> {
    let mut select = post::Entity::update_many().filter(Column::Id.eq(id));
    if let Some(is_del) = is_del {
        select = select.filter(Column::IsDel.eq(is_del));
    }
    if inc_hit.unwrap_or(true) {
        select = select.col_expr(Column::Hit, Expr::col(Column::Hit).add(1));
    }
    let res = select.exec_with_returning(db).await?;
    Ok(res.first().map(|model| model.to_owned()))
}

#[cfg(test)]
mod tests {
    use crate::dbaccess::select_record_total;

    #[tokio::test]
    async fn test_select_record_total() {
        let db = util::get_db_connection().await.unwrap();
        let res = select_record_total(&db, Some(2), None, None, None, None).await;
        dbg!(res.unwrap());
    }
}
