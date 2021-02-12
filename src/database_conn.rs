use rocket_contrib::databases::diesel;

pub use crate::diesel::dsl::*;
pub use crate::diesel::QueryDsl;
pub use crate::diesel::RunQueryDsl;
pub use crate::models::*;
pub use crate::schema::translations::dsl::*;

#[database("suryoyo_translate")]
pub struct SuryoyoTranslateDb(diesel::pg::PgConnection);
