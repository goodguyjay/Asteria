use crate::{asn_err, asn_info, handler};
use std::net::TcpListener;
use std::thread;

const BASE_ADDRESS: &str = "127.0.0.1";
const BASE_PORT: u16 = 5001;

pub fn start() {
    let listener =
        TcpListener::bind((BASE_ADDRESS, BASE_PORT)).expect("[ASN]: Failed to start server.");

    asn_info!("Listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handler::handle_client(stream));
            }

            Err(e) => asn_err!("Failed to establish connection: {}", e),
        }
    }
}
