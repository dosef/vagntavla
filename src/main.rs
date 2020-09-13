#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::{Config, Environment};
use rocket::http::ContentType;
use rocket::request::Form;
use rocket::response::content::Content;
use rocket::response::Redirect;
use rocket::{get, post, routes, uri, FromForm};
use rocket_contrib::serve::StaticFiles;
use std::process;
use tera::Context;
use tera::Tera;

mod configuration;
use configuration::Configuration;
mod vasttrafik;
use vasttrafik::Vasttrafik;

#[derive(FromForm)]
struct Stop {
    name: String,
}

#[get("/")]
fn index() -> Result<Content<String>, tera::Error> {
    let tera = match Tera::new("templates/**/*.html.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let context = Context::new();
    let index = tera.render("form.html.tera", &context);
    Ok(Content(ContentType::HTML, index.unwrap()))
}

#[post("/", data = "<stop>")]
fn show_upcoming(stop: Form<Stop>) -> Redirect {
    let cfg = Configuration::new().unwrap();
    let vt = match Vasttrafik::new(cfg) {
        Ok(value) => value,
        Err(message) => {
            println!("{}", message);
            process::exit(1);
        }
    };
    println!("{}", stop.name);
    let matching_stops = vt.get_stop_info(&stop.name).unwrap();
    Redirect::to(uri!(upcoming: &matching_stops[0].id))
}

#[get("/upcoming/<stop_id>")]
fn upcoming(stop_id: String) -> Result<Content<String>, tera::Error> {
    let cfg = Configuration::new().unwrap();
    let vt = match Vasttrafik::new(cfg) {
        Ok(value) => value,
        Err(message) => {
            println!("{}", message);
            process::exit(1);
        }
    };

    let upcoming = vt.get_upcoming_at_stop(&stop_id).unwrap();

    let tera = match Tera::new("templates/**/*.html.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("departures", &upcoming.departure_board.departure[0..5]);
    let index = tera.render("index.html.tera", &context);
    Ok(Content(ContentType::HTML, index.unwrap()))
}

fn main() {
    let cfg = Configuration::new().unwrap();

    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(cfg.port)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![index, upcoming, show_upcoming])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
