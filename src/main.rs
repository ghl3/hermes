
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

use std::thread;

use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;

use std::io::Read;
use std::str::FromStr;

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
    let data = stream.read_to_end(&mut buffer).unwrap();
    println!("{}", data);
}

fn main() {

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Listening on port: {:?}", args.flag_port);

    let addr: SocketAddr = SocketAddr::from_str(&format!("127.0.0.1:{}", args.flag_port)).unwrap();
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => { println!("ERROR: {:}", e); }
        }
    }
}
