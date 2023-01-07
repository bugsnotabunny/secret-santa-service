use std::{
    fs,
    net::TcpStream,
    io::prelude::*,
};
use crate::mucho_texto::*;

pub fn respond(mut stream: &TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
}

pub fn respond_not_allowed (stream: &TcpStream) {
    respond(
        stream,
        &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::UNSUPORTED_STANDARD)
                .unwrap()
                .as_str(),
        )
    )
}

pub fn respond_welcome(stream: &TcpStream) {
    respond(
        stream,
        &response::gen_response(
            status::OK,
            fs::read_to_string(json_msg_path::WELCOME)
                .unwrap()
                .as_str()
        )
    )
}

pub fn respond_unsuported_standard(stream: &TcpStream) {
    respond(
        stream,
        &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::UNSUPORTED_STANDARD)
                .unwrap()
                .as_str(),
        )
    )
}

pub fn respond_unsuported_command(stream: &TcpStream) {
    respond(
        stream,
        &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::UNSUPORTED_COMMAND)
                .unwrap()
                .as_str(),
        )
    )
}

pub fn respond_not_found(stream: &TcpStream) {
    respond(
        stream,
        &response::gen_response(
            status::NOT_FOUND,
            fs::read_to_string(json_msg_path::NO_CONTENT)
                .unwrap()
                .as_str(),
        ),
    );
}

pub fn respond_invalid_credentials(stream: &TcpStream) {
    respond(
        stream,
        &response::gen_response(
            status::ANAUTHORIZED,
            fs::read_to_string(json_msg_path::INVALID_CREDENTIALS)
                .unwrap()
                .as_str(),
        ),
    );
}

pub fn respond_deleted_successfully(stream: &TcpStream) {
    respond(
        stream,
        &response::gen_response(
            status::OK,
            json_msg_path::DELETED_SUCCESSFULLY
        )
    )
}
