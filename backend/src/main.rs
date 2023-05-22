#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;

use diesel::prelude::*;
use rocket::http::Method;
use rocket::{delete, get, post, put, routes, catch};
use rocket_codegen::catchers;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use serde_json::json;

use crate::database::establish_connection;
use crate::models::{Housing, NewHousing, TypeHousings, UpdateHousing};
use crate::schema::housings::dsl::housings;
use crate::schema::type_housings::dsl::type_housings;

mod database;
mod models;
mod schema;

#[get("/api/housings")]
pub fn get_housings() -> Json<JsonValue> {
    let mut connection = establish_connection();

    let housing_list = housings
        .load::<Housing>(&mut connection)
        .expect("Error loading housings");

    Json(JsonValue::from(json!(housing_list)))
}

#[delete("/api/housings/<id>")]
pub fn delete_housing(id: i32) -> Json<JsonValue> {
    let mut connection = establish_connection();

    diesel::delete(housings.find(id))
        .execute(&mut connection)
        .unwrap_or_else(|_| panic!("Unable to find housing {}", id));

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("Housing with ID {} has been deleted", id),
    })))
}

#[post("/api/housings", format = "json", data = "<new_housing>")]
pub fn create_housing(new_housing: Json<NewHousing>) -> Json<JsonValue> {
    let mut connection = establish_connection();
    let new_housing = NewHousing {
        street_name: new_housing.street_name,
        street_number: new_housing.street_number,
        floor: new_housing.floor,
        zip_code: new_housing.zip_code,
        square_meters: new_housing.square_meters,
        number_bathrooms: new_housing.number_bathrooms,
        number_bedrooms: new_housing.number_bedrooms,
        type_housing_id: new_housing.type_housing_id,
    };

    diesel::insert_into(housings)
        .values(&new_housing)
        .execute(&mut connection)
        .expect("Error saving new housing");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Housing has been created",

    })))
}

#[put("/api/housings/<id>", data = "<update_housing>")]
pub fn update_housing(id: i32, update_housing: Json<UpdateHousing>) -> Json<JsonValue> {
    let mut connection = establish_connection();

    let _update_housing = diesel::update(housings.find(id))
        .set(&update_housing.into_inner())
        .execute(&mut connection)
        .expect("Failed to update student");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("Housing {} has been updated", id),
    })))
}

#[get("/api/type-housings")]
pub fn get_type_housings() -> Json<JsonValue> {
    let mut connection = establish_connection();

    let type_housing_list = type_housings
        .load::<TypeHousings>(&mut connection)
        .expect("Error loading type of housings");

    Json(JsonValue::from(json!({
        "type_housings": type_housing_list,
    })))
}

#[catch(404)]
fn not_found() -> String {
    format!("Error:No matching routes found")
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                get_housings,
                delete_housing,
                create_housing,
                update_housing,
                get_type_housings,
            ],
        )
        .register(catchers![not_found])
        .attach(make_cors())
        .launch();
}

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![Method::Get, Method::Post, Method::Put, Method::Delete]
        .into_iter()
        .map(From::from)
        .collect();
    let allowed_headers = AllowedHeaders::all();

    CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS")
}
