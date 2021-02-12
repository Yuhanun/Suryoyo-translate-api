#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;

mod database_conn;
mod fetch;
mod models;
mod schema;
use database_conn::SuryoyoTranslateDb;

#[rocket::get("/<term>")]
fn get_translation(conn: SuryoyoTranslateDb, term: String) -> String {
    serde_json::to_string(&fetch::fetch_term_results(conn, &term)).unwrap()
}

fn main() {
    rocket::ignite()
        .attach(database_conn::SuryoyoTranslateDb::fairing())
        .mount("/api/v1/translate", rocket::routes![get_translation])
        .launch();
}
