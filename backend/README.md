# Housing Backend

This is the backend of the Housing App.

## Running the App
To run the app, use the following command:

```sh
cargo run
```

## API Documentation

The following is a list of the API endpoints available in this project:

- `GET /housings` - Get a list of all housings
- `POST /housings` - Create a new housing
- `PUT /housings/:id` - Update an existing housing
- `DELETE /housings/:id` - Delete a housing
- `GET /type-housings` - Get a list of all housings

### Example Requests

The following are some example requests that you can use to test the API:

- `GET /housings` - Get a list of all housings

```
curl http://localhost:8000/housings
```

- `POST /housings` - Create a new housing

```
curl -X POST http://localhost:8000/api/housings -H 'Content-Type: application/json' -d '{"street_name": "Lavalle", "street_number": 999, "floor": 1, "zip_code": 123, "square_meters": 45, "number_bathrooms": 2, "number_bedrooms": 2, "type_housing_id": 1}'
```

- `PUT /housings/1` - Update an existing housing

```
curl -X PUT http://localhost:8000/api/housings/1 -H 'Content-Type: application/json' -d '{"street_name": "Libertador", "street_number": 999, "floor": 1, "zip_code": 123, "square_meters": 45, "number_bathrooms": 2, "number_bedrooms": 2, "type_housing_id": 1}'
```

- `DELETE /housings/1` - Delete a housing

```
curl -X DELETE http://localhost:8000/api/housings/1
```

- `GET /type-housings` - Get a list of all Type housings

```
curl http://localhost:8000/type-housings
```

That's it! The Housing Backend is now running. Enjoy!
