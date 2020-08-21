use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use my_http::common::header;
use my_http::common::response::Response;
use my_http::common::status;
use my_http::header_map;
use my_http::server::{Config, Router, Server};
use my_http::server::ListenerResult::SendResponseArc;

mod logging_manager;

use logging_manager::*;
use std::path::Path;

fn main() -> Result<(), Error> {
    let mut server = Server::new(Config {
        addr: "0.0.0.0:80",
        connection_handler_threads: 5,
        read_timeout: Duration::from_millis(1000),
        tls_config: None,
    });

    server.router.route("/", file_router("./web/"));

    LoggingService::new(LoggingConfig { logging_directory: Path::new("./logs/"), max_dir_size: 100000 }).expect("Logging Service failed to start.");

    server.start()
}

fn file_router(directory: &'static str) -> Router {
    let mut router = Router::new();

    let cache: RwLock<HashMap<String, Arc<Response>>> = RwLock::new(HashMap::new());

    router.on_prefix("", move |uri, _| {
        let mut path = String::from(directory);
        path.push_str(uri);

        if path.ends_with("/") {
            path.push_str("index.html")
        }

        if let Some(response) = cache.read().unwrap().get(&path) { // read lock gets dropped after if statement
            return SendResponseArc(Arc::clone(response));
        }

        let response = Arc::new(file_response(&path));

        cache.write().unwrap().insert(path, Arc::clone(&response));

        SendResponseArc(response)
    });

    router
}

fn file_response(file_path: &str) -> Response {
    if let Ok(contents) = fs::read(file_path) {
        let headers = header_map![
            (header::CONTENT_LENGTH, contents.len().to_string()),
            (header::CONTENT_TYPE, get_content_type(file_path))
        ];

        return Response { status: status::OK, headers, body: contents };
    }
    return status::NOT_FOUND.into();
}

fn get_content_type(path: &str) -> &'static str {
    if path.ends_with(".ico") {
        return "image/x-icon";
    } else if path.ends_with(".js") {
        return "application/javascript";
    } else if path.ends_with(".svg") {
        return "image/svg+xml";
    } else if path.ends_with(".html") {
        return "text/html";
    } else if path.ends_with(".css") {
        return "text/css";
    } else if path.ends_with(".png") {
        return "image/png";
    } else if path.ends_with(".jpg") {
        return "image/jpeg";
    }
    "text/plain"
}