pub mod data;
pub mod group;
pub mod engrouped_user;
pub mod mucho_texto;
pub mod request_handler;
pub mod request_type;
pub mod respond;
pub mod user;

use std::net::TcpListener;

use crate::{data::Data, request_handler::handle_connection};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut data = Data::build();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(&stream, &mut data);
    }
}
