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
        type_housing_id -> Text,
    }
}
