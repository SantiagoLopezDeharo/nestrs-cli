use crate::primitives::http::request::Request;
use crate::primitives::http::response::Response;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::OnceLock;

pub mod init;
pub use init::init_routes;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
pub type Handler =
    Box<dyn for<'a> Fn(&'a Request, &'a RouteParams) -> BoxFuture<'a, Response> + Send + Sync>;

#[derive(Debug, Default)]
pub struct RouteParams {
    params: HashMap<String, String>,
}

impl RouteParams {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }
}

pub struct Route {
    pub method: &'static str,
    pub path: &'static [&'static str],
    pub handler: Handler,
}

impl Route {
    pub fn new(method: &'static str, path: &'static [&'static str], handler: Handler) -> Self {
        Self {
            method,
            path,
            handler,
        }
    }
}

#[macro_export]
macro_rules! route {
    ($handler:path) => {
        Box::new(|req, params| Box::pin($handler(req, params)))
    };
}

static ROUTES: OnceLock<Vec<Route>> = OnceLock::new();

pub fn init(routes: Vec<Route>) {
    let _ = ROUTES.set(routes);
}

pub fn routes() -> &'static [Route] {
    ROUTES.get().map(|r| r.as_slice()).unwrap_or(&[])
}

pub async fn route(request: &Request) -> Response {
    let path = request.url.split('?').next().unwrap_or("");

    let segments: Vec<&str> = path
        .trim_matches('/')
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let routes = routes();
    let mut path_matched = false;

    for route_def in routes {
        let params = match path_match_params(route_def.path, &segments) {
            Some(params) => params,
            None => continue,
        };
        path_matched = true;
        if route_def.method == request.method {
            return (route_def.handler)(request, &params).await;
        }
    }

    if path_matched {
        return method_not_allowed();
    }

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    Response {
        status_code: 404,
        headers,
        body: "Not Found".to_string(),
    }
}

fn path_match_params(pattern: &[&str], segments: &[&str]) -> Option<RouteParams> {
    if pattern.len() != segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (p, s) in pattern.iter().zip(segments.iter()) {
        if let Some(name) = p.strip_prefix(':') {
            params.insert(name.to_string(), (*s).to_string());
            continue;
        }
        if p != s {
            return None;
        }
    }

    Some(RouteParams { params })
}

fn method_not_allowed() -> Response {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    Response {
        status_code: 405,
        headers,
        body: "Method Not Allowed".to_string(),
    }
}
