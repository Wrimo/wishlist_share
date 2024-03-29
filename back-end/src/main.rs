use std::{result, vec};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Status};
use rocket::serde::{json::Json, Serialize};
use rocket::{serde, Request, Response, State};
use serde_json;
use sqlx::{postgres::PgPoolOptions, Column, Row};

pub mod db;

use db::generated_procedures::{get_person_family, get_person_wishlist_items, set_purchased};
use db::generated_types::{get_person_familyData, wishlistitemData};

#[macro_use]
extern crate rocket;

pub struct CORS;

struct Config {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index(state: &State<Config>) -> String {
    format!("The config value is: {}", state.pool.is_closed());
    "hello".to_string()
}

#[get("/family/<person_id>")]
async fn get_family(
    state: &State<Config>,
    person_id: i32,
) -> (Status, Json<Vec<get_person_familyData>>) {
    let result = get_person_family(&state.pool, person_id).await;

    match result {
        Ok(r) => return (Status::Ok, Json(r)),
        Err(_e) => {
            return (
                Status::InternalServerError,
                Json(Vec::<get_person_familyData>::new()),
            )
        }
    };
}

#[get("/wishlist/<person_id>")]
async fn get_wishlist(
    state: &State<Config>,
    person_id: i32,
) -> (Status, Json<Vec<wishlistitemData>>) {
    let result = get_person_wishlist_items(&state.pool, person_id).await;

    match result {
        Ok(r) => return (Status::Ok, Json(r)),
        Err(_e) => {
            return (
                Status::InternalServerError,
                Json(Vec::<wishlistitemData>::new()),
            )
        }
    };
}

#[post("/wishlist_item/<item_id>")]
async fn update_purchased(
    state: &State<Config>,
    item_id: i32,
) -> Status {
    let result: Result<(), sqlx::Error> = set_purchased(&state.pool, item_id).await; 
    match result { 
        Ok(_r) => return Status::Ok, 
        Err(_e) => return Status::InternalServerError

    }
}

#[launch]
async fn rocket() -> _ {
    let config = Config {
        pool: PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://brrennan:LifeEnder@localhost:5432/brrennan")
            .await
            .unwrap(),
    };

    rocket::build()
        .mount("/", routes![index, get_family, get_wishlist, update_purchased])
        .attach(CORS)
        .manage(config)
}
