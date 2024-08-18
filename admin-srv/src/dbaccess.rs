use anyhow::{anyhow, Result};
use entity::entity::admin;
use entity::entity::admin::Column;
use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

pub async fn count_admin_by_email(db: &DatabaseConnection, email: &str) -> Result<i32> {
    let row_affected = admin::Entity::find()
        .filter(Column::Email.eq(email))
        .count(db)
        .await?;
    Ok(row_affected as i32)
}

pub async fn count_admin_by_id(db: &DatabaseConnection, id: i32) -> Result<i32> {
    let row_affected = admin::Entity::find_by_id(id).count(db).await?;
    Ok(row_affected as i32)
}

pub async fn select_admin_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<admin::Model>> {
    let res = admin::Entity::find()
        .filter(Column::Email.eq(email))
        .one(db)
        .await?;
    Ok(res)
}

pub async fn select_admin_by_id(
    db: &DatabaseConnection,
    id: i32,
    is_del: Option<bool>,
) -> Result<Option<admin::Model>> {
    let mut select = admin::Entity::find_by_id(id);
    if let Some(is_del) = is_del {
        select = select.filter(Column::IsDel.eq(is_del));
    }
    Ok(select.one(db).await?)
}

pub async fn update_admin_pwd(db: &DatabaseConnection, id: i32, new_pwd: &str) -> Result<i32> {
    let res = admin::Entity::update_many()
        .filter(Column::Id.eq(id))
        .col_expr(Column::Password, Expr::value(new_pwd))
        .exec(db)
        .await?;
    Ok(res.rows_affected as i32)
}

pub async fn select_admins(
    db: &DatabaseConnection,
    email: &Option<String>,
    is_del: &Option<bool>,
) -> Result<Vec<admin::Model>> {
    let mut select = admin::Entity::find();
    if let Some(email) = email {
        select = select.filter(Column::Email.contains(email));
    }
    if let Some(is_del) = is_del {
        select = select.filter(Column::IsDel.eq(*is_del));
    }
    Ok(select.all(db).await?)
}

pub async fn insert_admin(db: &DatabaseConnection, email: &str, password: &str) -> Result<i32> {
    let new_admin = admin::ActiveModel {
        email: Set(email.to_owned()),
        password: Set(password.to_owned()),
        ..Default::default()
    };
    let res = admin::Entity::insert(new_admin).exec(db).await?;
    Ok(res.last_insert_id)
}

pub async fn update_admin_del(db: &DatabaseConnection, id: i32) -> Result<bool> {
    let res = admin::Entity::update_many()
        .filter(Column::Id.eq(id))
        .col_expr(Column::IsDel, Expr::col(Column::IsDel).not())
        .exec_with_returning(db)
        .await?;
    match res.first() {
        Some(row) => Ok(row.is_del),
        None => Err(anyhow!("No such admin")),
    }
}
