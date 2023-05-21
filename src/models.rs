use diesel::{prelude::*};
use serde_derive::{Deserialize, Serialize};

use crate::schema::housings;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(TypeHousings, foreign_key = type_housing_id))]
#[diesel(table_name = housings)]
pub struct Housing {
    pub id: i32,
    pub street_name: String,
    pub street_number: i32,
    pub floor: i32,
    pub zip_code: i32,
    pub square_meters: i32,
    pub number_bathrooms: i32,
    pub number_bedrooms: i32,
    pub type_housing_id: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = housings)]
pub struct NewHousing<'a> {
    pub street_name: &'a str,
    pub street_number: i32,
    pub floor: i32,
    pub zip_code: i32,
    pub square_meters: i32,
    pub number_bathrooms: i32,
    pub number_bedrooms: i32,
    pub type_housing_id: i32,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = housings)]
pub struct UpdateHousing {
    pub street_name: Option<String>,
    pub street_number: Option<i32>,
    pub floor: Option<i32>,
    pub zip_code: Option<i32>,
    pub square_meters: Option<i32>,
    pub number_bathrooms: Option<i32>,
    pub number_bedrooms: Option<i32>,
    pub type_housing_id: Option<i32>,
}

#[derive(Queryable, Serialize)]
pub struct TypeHousings {
    pub id: i32,
    pub name: String
}
