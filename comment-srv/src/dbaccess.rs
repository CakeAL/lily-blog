use anyhow::{anyhow, Result};
use entity::entity::comment::{self, Column};
use sea_orm::prelude::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn insert_comment(
    db: &DatabaseConnection,
    post_id: i32,
    name: String,
    hashed_email: String,
    content: String,
) -> Result<i32> {
    let new_comment = comment::ActiveModel {
        post_id: Set(post_id),
        name: Set(name),
        hashed_email: Set(Some(hashed_email)),
        content: Set(Some(content)),
        ..Default::default()
    };
    let res = comment::Entity::insert(new_comment).exec(db).await?;
    Ok(res.last_insert_id)
}

pub async fn select_comments(db: &DatabaseConnection, post_id: i32) -> Result<Vec<comment::Model>> {
    let res = comment::Entity::find()
        .filter(Column::PostId.eq(post_id))
        .filter(Column::IsDel.eq(false)) // 默认查询未被删除的评论
        .all(db)
        .await?;
    Ok(res)
}
pub async fn update_comment_del(db: &DatabaseConnection, id: i32) -> Result<bool> {
    let res = comment::Entity::update_many()
        .filter(Column::Id.eq(id))
        .col_expr(Column::IsDel, Expr::col(Column::IsDel).not())
        .exec_with_returning(db)
        .await?;

    match res.first() {
        Some(row) => Ok(row.is_del),
        None => Err(anyhow!("No such comment")),
    }
}
