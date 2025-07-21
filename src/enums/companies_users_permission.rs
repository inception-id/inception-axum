use crate::schema;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, AsExpression, Deserialize, Serialize, Clone, FromSqlRow)]
#[diesel(sql_type = schema::sql_types::CompaniesUsersPermission)]
#[serde(rename_all = "lowercase")]
pub enum CompanyUserPermission {
    Owner,
    Edit,
    View,
}

impl ToSql<schema::sql_types::CompaniesUsersPermission, Pg> for CompanyUserPermission {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            CompanyUserPermission::Owner => out.write_all(b"owner")?,
            CompanyUserPermission::Edit => out.write_all(b"edit")?,
            CompanyUserPermission::View => out.write_all(b"view")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::CompaniesUsersPermission, Pg> for CompanyUserPermission {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"owner" => Ok(CompanyUserPermission::Owner),
            b"edit" => Ok(CompanyUserPermission::Edit),
            b"view" => Ok(CompanyUserPermission::View),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
