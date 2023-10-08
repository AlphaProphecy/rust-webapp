#[macro_use] extern crate rocket;

use rocket_sync_db_pools::{database, diesel};
use rocket::serde::json::Json;
use diesel::prelude::*;
use dotenv::dotenv;
use models::Post;

use diesel::MysqlConnection;

mod models;
mod schema;

#[database("mysql")]
pub struct Db(MysqlConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, worlds!"
}

#[get("/test/<name>")]
fn test(name: &str) -> String {
    format!("Hello sssssssssssssname, {}!", name)
}

#[get("/posts")]
async fn get_posts(conn: Db) -> Json<Vec<Post>> {
    use schema::posts::dsl::*;

    let results = conn.run(|c| {
        posts
            .filter(published.eq(true))
            .select(Post::as_select())
            .get_results(c)
            .expect("Error loading posts")
    }).await;

    Json(results)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(Db::fairing())
        .mount("/", routes![index, test, get_posts])
}