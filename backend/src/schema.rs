// @generated automatically by Diesel CLI.

diesel::table! {
    housings (id) {
        id -> Integer,
        street_name -> Text,
        street_number -> Integer,
        floor -> Integer,
        zip_code -> Integer,
        square_meters -> Integer,
        number_bathrooms -> Integer,
        number_bedrooms -> Integer,
        type_housing_id -> Integer,
    }
}

diesel::table! {
    type_housings (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(housings -> type_housings (type_housing_id));

diesel::allow_tables_to_appear_in_same_query!(housings, type_housings,);
