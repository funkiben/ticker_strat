use std::io::Error;
use my_http::server::{Server, Config};
use std::time::Duration;
use my_http::common::header::{CONTENT_LENGTH, HeaderMapOps};
use my_http::common::status::OK_200;
use my_http::common::response::Response;
use my_http::server::ListenerResult::SendResponse;

fn main() -> Result<(), Error> {
    let mut server = Server::new(Config {
        addr: "0.0.0.0:80",
        connection_handler_threads: 5,
        read_timeout: Duration::from_millis(10000),
    });

    server.router.on_prefix("/", |_, _| {
        println!("Received a request");
        let message = b"I work!";
        SendResponse(Response {
            status: OK_200,
            headers: HeaderMapOps::from(vec![(CONTENT_LENGTH, message.len().to_string())]),
            body: message.to_vec(),
        })
    });

    println!("Starting server");
    server.start()
}