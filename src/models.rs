use crate::schema::users;
use crate::validate_email;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub numbers: Numbers,
}

#[derive(Deserialize, Validate)]
pub struct NewUser {
    pub name: String,
    #[validate(custom(
        function = "validate_email",
        arg = "(&'v_a diesel::pg::PgConnection, FnOnce(&str, &diesel::pg::PgConnection) -> Result<Option<User>, diesel::result::Error>)"
    ))]
    pub email: String,
    #[validate]
    pub numbers: Numbers,
}

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone, Default, Validate)]
#[sql_type = "Jsonb"]
pub struct Numbers {
    #[validate(length(min = 2))]
    pub home: String,
    // #[validate(custom(function = "validate_phone", arg = "(&'v_a diesel::pg::PgConnection, FnOnce(&str, &diesel::pg::PgConnection) -> Result<Option<User>, diesel::result::Error>)"))]
    pub work: String,
    // #[validate(custom(function = "validate_phone", arg = "(&'v_a diesel::pg::PgConnection, FnOnce(&str, &diesel::pg::PgConnection) -> Result<Option<User>, diesel::result::Error>)"))]
    pub cell: String,
}

use diesel::backend::Backend;
use diesel::{
    deserialize::{self, FromSql},
    pg::{types::sql_types::Jsonb, Pg},
    serialize::{self, IsNull, Output, ToSql},
};
use std::io::Write;

impl FromSql<Jsonb, Pg> for Numbers {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        if bytes[0] != 1 {
            return Err("Unsupported JSONB encoding version".into());
        }

        serde_json::from_slice(&bytes[1..]).map_err(Into::into)
    }
}

impl ToSql<Jsonb, Pg> for Numbers {
    fn to_sql<'a, W: Write>(&self, out: &mut Output<'a, W, Pg>) -> serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}
