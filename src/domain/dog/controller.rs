use std::collections::HashMap;

use crate::primitives::http::request::Request;
use crate::primitives::http::response::Response;
use crate::route;
use crate::routing::{Route, RouteParams};

use super::repo::DogRepo;
use super::service::DogService;

pub struct DogController;

impl DogController {
    pub fn routes() -> Vec<Route> {
        vec![
            Route::new("GET", &["dog"], route!(DogController::get_all)),
            Route::new("POST", &["dog"], route!(DogController::create)),
            Route::new("GET", &["dog", ":id"], route!(DogController::get_one)),
            Route::new("PUT", &["dog", ":id"], route!(DogController::update)),
            Route::new("DELETE", &["dog", ":id"], route!(DogController::delete)),
        ]
    }

    pub async fn get_all(_request: &Request, _params: &RouteParams) -> Response {
        let service = DogService::new(DogRepo::new());

        let body = service.speak();

        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        Response {
            status_code: 200,
            headers,
            body,
        }
    }

    pub async fn get_one(_request: &Request, params: &RouteParams) -> Response {
        let service = DogService::new(DogRepo::new());
        let _id = params.get("id").unwrap_or("");
        let body = service.speak();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 200,
            headers,
            body,
        }
    }

    pub async fn create(_request: &Request, _params: &RouteParams) -> Response {
        let service = DogService::new(DogRepo::new());
        let body = service.speak();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 201,
            headers,
            body,
        }
    }

    pub async fn update(_request: &Request, params: &RouteParams) -> Response {
        let service = DogService::new(DogRepo::new());
        let _id = params.get("id").unwrap_or("");
        let body = service.speak();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 200,
            headers,
            body,
        }
    }

    pub async fn delete(_request: &Request, params: &RouteParams) -> Response {
        let service = DogService::new(DogRepo::new());
        let _id = params.get("id").unwrap_or("");
        let body = service.speak();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response {
            status_code: 200,
            headers,
            body,
        }
    }
}
