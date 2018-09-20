# API Documentation

The API should only expose a few functions:

| Method   | Endpoint   | Description                               |
|----------|------------|-------------------------------------------|
| **GET**  | /api/value | Should return a `json` data for the value |
| **POST** | /api/value | Should set a new value                    |

## Tools

Since [rocket](https://rocket.rs) framework already has the RESTful toolkit readily available, you can continue to use that.

~~The adventurous one can opt to use [actix](https://actix.rs) as it is more to be used in the production environment.~~


