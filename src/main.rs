#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, worlds!"
}

#[get("/test/<name>")]
fn test(name: &str) -> String {
    format!("Hello name, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, test])
}