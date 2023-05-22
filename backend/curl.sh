#!/bin/bash

# Get all housings
curl http://localhost:8000/api/housings

# Create a new housing
curl -X POST http://localhost:8000/api/housings -H 'Content-Type: application/json' -d '{"street_name": "Lavalle", "street_number": 999, "floor": 1, "zip_code": 123, "square_meters": 45, "number_bathrooms": 2, "number_bedrooms": 2, "type_housing_id": 1}'

# Update a housing with ID 1
curl -X PUT http://localhost:8000/api/housings/1 -H 'Content-Type: application/json' -d '{"street_name": "Libertador", "street_number": 999, "floor": 1, "zip_code": 123, "square_meters": 45, "number_bathrooms": 2, "number_bedrooms": 2, "type_housing_id": 1}'

# Delete a housing with ID 1
curl -X DELETE http://localhost:8000/api/housings/1

# Get all type housings
curl http://localhost:8000/api/type-housings
