#![allow(unused_imports, dead_code)]

pub mod common;

use chrono::Utc;
pub use common::{features::*, setup::*, TestContext};
use pretty_assertions::assert_eq;
use sea_orm::{entity::prelude::*, entity::*, DatabaseConnection};
use serde_json::json;
use sea_query::ExprTrait;
#[cfg(feature = "with-time")]
use time::{Duration, macros::{time, date, format_description}};

#[sea_orm_macros::test]
#[cfg(all(feature = "with-time", feature = "sqlx-postgres"))]
async fn main() -> Result<(), DbErr> {
    let ctx = TestContext::new("audit_log_range_tests").await;
    create_tables(&ctx.db).await?;
    insert_entry(&ctx.db).await?;
    ctx.delete().await;

    Ok(())
}
#[cfg(all(feature = "with-time", feature = "sqlx-postgres"))]
pub async fn insert_entry(db: &DatabaseConnection) -> Result<(), DbErr> {
    let format = time::macros::format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
             sign:mandatory]:[offset_minute]"
    );
    let string = "2020-01-01 02:02:02 +08:00";
    let timestamp = time::OffsetDateTime::parse(string, &format).unwrap();
    let later = timestamp.checked_add(Duration::days(2)).unwrap();
    let range = pgrange::PgRange {
        start: std::ops::Bound::Included(timestamp),
        end: std::ops::Bound::Excluded(later),
    };


    let log = audit_log::Model {
        id: 1,
        business_key: "FOO".into(),
        effective: range,
    };

    let log_am = log.clone().into_active_model();

    let result = log_am.insert(db).await?;

    assert_eq!(result, log);

    let json = audit_log::Entity::find()
        .filter(audit_log::Column::Id.eq(log.id))
        .into_json()
        .one(db)
        .await?;

    assert_eq!(
        json,
        Some(json!({
            "id": log.id,
            "business_key": log.business_key,
            "effective": log.effective,
            "effective": {
                "start":{
                    "Included": log.effective.start().unwrap().to_utc().format(&format).unwrap(),
                },
                "end":{
                    "Excluded": log.effective.end().unwrap().to_utc().format(&format).unwrap(),
                }
            },
        }))
    );

    Ok(())
}
