-- Your SQL goes here
CREATE TABLE "housings" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "street_name" TEXT NOT NULL,
  "street_number" INTEGER NOT NULL,
  "floor" INTEGER NOT NULL,
  "zip_code" INTEGER NOT NULL,
  "square_meters" INTEGER NOT NULL,
  "number_bathrooms" INTEGER NOT NULL,
  "number_bedrooms" INTEGER NOT NULL,
  "type_housing_id" TEXT NOT NULL
);
