use std::io::Error;
use std::sync::Arc;
use std::time::Duration;

use my_http::common::header::CONTENT_LENGTH;
use my_http::common::response::Response;
use my_http::common::status;
use my_http::header_map;
use my_http::server::{Config, Server};
use my_http::server::ListenerResult::SendResponseArc;

fn main() -> Result<(), Error> {
    let mut server = Server::new(Config {
        addr: "0.0.0.0:80",
        connection_handler_threads: 5,
        read_timeout: Duration::from_millis(1000),
    });

    let response_message = b"I work!";
    let response = Response {
        status: status::OK,
        headers: header_map![
            (CONTENT_LENGTH, response_message.len().to_string())
        ],
        body: response_message.to_vec(),
    };

    let response = Arc::new(response);
    server.router.on_prefix("/", move |_, _| {
        println!("Received a request");
        SendResponseArc(Arc::clone(&response))
    });

    println!("Starting server");
    server.start()
}