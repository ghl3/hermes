
extern crate rustc_serialize;
extern crate docopt;
extern crate tiny_http;

use docopt::Docopt;

use std::thread;

use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;

use std::io::Read;
use std::io::Cursor;

use std::str::FromStr;


use tiny_http::{Server, Request, Response, StatusCode, Method, Header};



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


fn handle_request(mut request: Request) {
    //println!("Headers: {}", request.headers());
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    println!("Body: {}", content);

    let response = match request.method() {
        &Method::Get => handle_get(&request),
        &Method::Post => handle_post(&request),
        &Method::Put => handle_put(&request),
        &Method::Delete => handle_delete(&request),
        _ => handle_nonstandard_request(&request)
    };

    request.respond(response);
}


//static headers: Vec<tiny_http::Header> = Vec::new();
//static header: Header = tiny_http::Header::from_bytes(&b"Content-Type", &b"text/plain").unwrap();



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

fn handle_get(request: &Request) -> Response<Cursor<Vec<u8>>> {
    println!("GET");
    http_response(StatusCode(200), "COOL")
    //Response::new(StatusCode(200), headers, "COOL", Option::None, Option::None)
}
fn handle_post(request: &Request) -> Response<Cursor<Vec<u8>>> {
    println!("POST");
    http_response(StatusCode(200), "COOL")
}
fn handle_put(request: &Request) -> Response<Cursor<Vec<u8>>> {
    println!("PUT");
    http_response(StatusCode(200), "COOL")
}
fn handle_delete(request: &Request) -> Response<Cursor<Vec<u8>>> {
    println!("DELETE");
    http_response(StatusCode(200), "COOL")
}
fn handle_nonstandard_request(request: &Request) -> Response<Cursor<Vec<u8>>> {
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

/*
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("ERROR: {:}", e);
            }
        }
    }
*/
}
