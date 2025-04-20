use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "audit_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub business_key: String,
    pub foo: Option<String>,
    pub bar: Option<String>,
    pub effective: TimeDateTime,
    pub asserted: TimeDateTimeWithTimeZone,
    pub effective_range: pgrange::PgRange<TimeDateTime>,
    pub asserted_range: pgrange::PgRange<TimeDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
