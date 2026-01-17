use crate::domain::dog::controller::DogController;
use crate::routing::Route;

pub fn init_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.extend(DogController::routes());

    routes
}
