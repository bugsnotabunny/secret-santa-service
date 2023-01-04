pub mod group;
pub mod user;
pub mod data;
pub mod mucho_texto;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
};

use mucho_texto::json_msg_path;

use crate::{
    data::Data,
    mucho_texto::{status, response}
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut data = Data::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(&stream, &mut data);
    }
}

fn respond(mut stream: &TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: &TcpStream, data: &mut Data) {
    static MAX_CONTENT_LEN: usize = 500;

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let content_type_raw = http_request
        .iter()
        .find(|&s| s.starts_with("Content-Type: application/json"));

    if content_type_raw.is_none() {
        respond(stream, &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_TYPE).unwrap().as_str()
        ));
        return;
    }

    let content_len_str = http_request
        .iter()
        .find(|&s| s.starts_with("Content-Length: "));
    if content_len_str.is_none() {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
        return;
    }

    let content_len_raw = content_len_str.unwrap().split_whitespace().last();
    if content_len_raw.is_none() {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
        return;
    }

    let content_len_wrapped = content_len_raw.unwrap().parse::< usize >();
    if content_len_raw.is_none() {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
        return;
    }

    let content_len = content_len_wrapped.unwrap();

    if content_len > MAX_CONTENT_LEN {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
        return;
    }

    let credentials_wrapped = http_request
        .iter()
        .find(|&s| s.starts_with("Authorization: basic "));
    if credentials_wrapped.is_none() {
        respond(stream, &response::gen_response(
            status::ANAUTHORIZED,
            fs::read_to_string(json_msg_path::INVALID_CREDENTIALS).unwrap().as_str()
        ));
        return;
    }

    let credentials = credentials_wrapped.unwrap();

    let user = data.login(credentials);
    if user.is_none() {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
        return;
    }

    let request_line_splited = http_request.first().unwrap().split_whitespace().collect::<Vec<_>>();

    let http_standard = request_line_splited.last().unwrap();

    if **http_standard != *"HTTP/1.1" {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
        return;
    }

    let type_enum = request_line_splited[0];

    let request_path_preformated = request_line_splited[1..(request_line_splited.len()-1)]
        .join(" ");
    let request_path = request_path_preformated
        .split('/')
        .collect::<Vec<_>>();

    if request_path.len() < 1 { // GET /
        respond(stream, response::WELCOME_TXT);
        return;
    }

    if *type_enum == *"GET" {
        handle_get_request(stream, data, &request_path)
    } else if *type_enum == *"POST" {

    } else {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
    }
}

fn handle_get_request(stream: &TcpStream, data: &Data, request_path: &Vec<&str>) {
    if request_path[0] == "users" {
        if request_path.len() < 2 {
            let users = data.get_users();
            let json_users = serde_json::to_string(&users).unwrap();
            respond(stream, response::gen_response(status::OK, json_users.as_str()).as_str());
            return;
        }
        let user = data.get_user(&request_path[1].to_string());
        if user.is_none() {
            respond(stream, response::gen_response(status::NOT_FOUND, "").as_str());
            return;
        }
        let json_user = serde_json::to_string(user.unwrap()).unwrap();
        respond(stream, response::gen_response(status::OK, json_user.as_str()).as_str());
    } else if request_path[0] == "groups" {
        if request_path.len() < 2 {
            let groups = data.get_groups();
            let json_groups = serde_json::to_string(&groups).unwrap();
            respond(stream, response::gen_response(status::OK, json_groups.as_str()).as_str());
            return;
        }
        let group = data.get_group(&request_path[1].to_string());
        if group.is_none() {
            respond(stream, response::gen_response(status::NOT_FOUND, "").as_str());
            return;
        }
        let json_group = serde_json::to_string(&group).unwrap();
        respond(stream, response::gen_response(status::OK, json_group.as_str()).as_str());
    }
}
