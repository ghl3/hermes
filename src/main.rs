
extern crate rustc_serialize;
extern crate docopt;
extern crate tiny_http;
extern crate url;

use std::thread;
use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::io::Read;
use std::io::Cursor;
use std::str::FromStr;

use rustc_serialize::json::Json;

use tiny_http::{Server, Request, Response, StatusCode, Method, Header};

use docopt::Docopt;

mod router;
//mod handler;


const USAGE: &'static str = "
Naval Fate.

Usage:
  rust_redis [--port=<p>]
  naval_fate.py (-h | --help)
  naval_fate.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --port=<p>    Port to listen on [default: 5001]
";


#[derive(Debug, RustcDecodable)]
struct Args {
    flag_port: i32
}


fn handle_client(mut stream: TcpStream) {
    let mut buffer = Vec::<u8>::new();
    let _ = stream.read_to_end(&mut buffer).unwrap();
    println!("{}", String::from_utf8(buffer).unwrap());
}


enum ParsedRequest {
    Unknown,
    GetRequest(String),
    PostJson(Json),
    PutJson(Json),
    DeleteRequest(String)
}


fn parse_request(request: &mut Request) -> ParsedRequest {
    let response = match request.method() {
        &Method::Get => ParsedRequest::GetRequest(request.url().to_string()),
        &Method::Post => ParsedRequest::PostJson(get_json(request)),
        &Method::Put => ParsedRequest::PutJson(get_json(request)),
        &Method::Delete => ParsedRequest::DeleteRequest(request.url().to_string()),
        _ => ParsedRequest::Unknown
    };
    response
}




fn handle_request(mut request: Request) {

    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    println!("Body: {}", content);

    let response = match parse_request(&mut request) {
        ParsedRequest::GetRequest(url) => handle_get(url), //http_response(StatusCode(500), "Unsupported"),
        ParsedRequest::PostJson(json) => handle_post(json),
        ParsedRequest::PutJson(json) => handle_put(json),
        ParsedRequest::DeleteRequest(url) => handle_delete(url),
        ParsedRequest::Unknown => http_response(StatusCode(400), "Send me JSON, fuck face")
    };
    request.respond(response);
}

fn get_json(request: &mut Request) -> Json {
    println!("Getting Json");
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    let json: Json = content.parse().unwrap();
    println!("GOT Json");
    json
}


fn http_response<S>(status: StatusCode, data: S) -> Response<Cursor<Vec<u8>>> where S: Into<String> {

        let data = data.into();
        let data_len = data.len();

        Response::new(
            status,
            vec![
                Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=UTF-8"[..]).unwrap()
                    ],
            Cursor::new(data.into_bytes()),
            Some(data_len),
            None,
            )
}

fn handle_get(url: String) -> Response<Cursor<Vec<u8>>> {
    println!("GET");
    http_response(StatusCode(200), "COOL")
}
fn handle_post(request: Json) -> Response<Cursor<Vec<u8>>> {
    println!("POST");
    http_response(StatusCode(200), "COOL")
}
fn handle_put(request: Json) -> Response<Cursor<Vec<u8>>> {
    println!("PUT");
    http_response(StatusCode(200), "COOL")
}
fn handle_delete(url: String) -> Response<Cursor<Vec<u8>>> {
    println!("DELETE");
    http_response(StatusCode(200), "COOL")
}
fn handle_nonstandard_request(request: Json) -> Response<Cursor<Vec<u8>>> {
    println!("WTF!!!");
    http_response(StatusCode(200), "COOL")
}


fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Listening on port: {:?}", args.flag_port);

    let addr: SocketAddr = SocketAddr::from_str(&format!("127.0.0.1:{}", args.flag_port)).unwrap();

    let server = Server::http(addr).unwrap();


    loop {
        // blocks until the next request is received
        let request = match server.recv() {
            Ok(rq) => handle_request(rq),
            Err(e) => { println!("error: {}", e); break }
        };

        // do something with the request
        // ...
        }
}
