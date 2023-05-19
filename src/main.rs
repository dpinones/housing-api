#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
extern crate diesel;

use diesel::prelude::*;
use rocket::delete;
use rocket::get;
use rocket::post;
use rocket::put;
use rocket::routes;
use rocket_contrib::json::{Json, JsonValue};
use serde_json::json;

use crate::database::establish_connection;
use crate::models::{NewHousing, Housing, UpdateHousing};
use crate::schema::housings::dsl::housings;

mod schema;
mod database;
mod models;


#[get("/housings")]
pub fn get_housings() -> Json<JsonValue> {
    let mut connection = establish_connection();

    let housing_list = housings.load::<Housing>(&mut connection).expect("Error loading housings");

    Json(JsonValue::from(json!({
        "housings": housing_list,
    })))
}


#[delete("/housings/<id>")]
pub fn delete_housing(id: i32) -> Json<JsonValue> {
    let mut connection = establish_connection();

    diesel::delete(housings.find(id)).execute(&mut connection).expect(&format!("Unable to find housing {}", id));

    Json(JsonValue::from(json!({
        "status": "success",
        "message": format!("Housing with ID {} has been deleted", id),
    })))
}


#[post("/housings", format = "json", data = "<new_housing>")]
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

#[put("/housings/<id>", data = "<update_housing>")]
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

fn main() {
    rocket::ignite().mount("/", routes![
        get_housings,
        delete_housing,
        create_housing,
        update_housing,
    ]).launch();
}
