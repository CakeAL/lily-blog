use anyhow::{anyhow, Result};
use entity::entity::tag::{self, Column};
use sea_orm::{prelude::*, Set};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

pub async fn select_tag_exists_by_id(db: &DatabaseConnection, id: i32) -> Result<i32> {
    let rows = entity::Tag::find_by_id(id).count(db).await?;
    Ok(rows as i32)
}

pub async fn select_tag_exists_by_name(db: &DatabaseConnection, name: &str) -> Result<i32> {
    let rows = entity::Tag::find()
        .filter(Column::Name.eq(name))
        .count(db)
        .await?;
    Ok(rows as i32)
}

pub async fn insert_new_tag(db: &DatabaseConnection, name: &str) -> Result<i32> {
    let new_tag = tag::ActiveModel {
        name: Set(name.to_owned()),
        ..Default::default()
    };
    let res = tag::Entity::insert(new_tag).exec(db).await?;
    Ok(res.last_insert_id)
}

pub async fn update_tag(db: &DatabaseConnection, id: i32, name: &str) -> Result<u64> {
    let res = entity::Tag::update_many()
        .filter(Column::Id.eq(id))
        .col_expr(Column::Name, Expr::value(name))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

pub async fn select_tags(
    db: &DatabaseConnection,
    name: &Option<String>,
    is_del: &Option<bool>,
) -> Result<Vec<tag::Model>> {
    let mut select = entity::Tag::find()
        .filter(Column::Name.contains(name.to_owned().unwrap_or("".to_string())));

    if let Some(is_del) = is_del {
        select = select.filter(Column::IsDel.eq(*is_del));
    }

    let tags = select.all(db).await?;
    Ok(tags)
}

pub async fn update_tag_del(db: &DatabaseConnection, id: i32) -> Result<bool> {
    let res = entity::Tag::update_many()
        .filter(Column::Id.eq(id))
        .col_expr(Column::IsDel, Expr::col(Column::IsDel).not())
        .exec_with_returning(db)
        .await?;

    match res.get(0) {
        Some(row) => Ok(row.is_del),
        None => Err(anyhow!("No such tag")),
    }
}

pub async fn select_tag_info(
    db: &DatabaseConnection,
    id: i32,
    is_del: &Option<bool>,
) -> Result<Option<tag::Model>> {
    let mut select = entity::Tag::find_by_id(id);

    if let Some(is_del) = is_del {
        select = select.filter(Column::IsDel.eq(*is_del));
    }

    let tag = select.one(db).await?;
    Ok(tag)
}
