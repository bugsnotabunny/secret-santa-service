use std::{
    io::{prelude::*, BufReader},
    net::{TcpStream},
    fs,
    str::FromStr
};

use crate::{
    data::Data,
    user::User,
    mucho_texto::*,
    request_type::RequestType
};

pub fn handle_connection(mut stream: &TcpStream, data: &mut Data) {

    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::< Vec< _ > >();

    if !check_content_type(&http_request) {
        respond(stream, &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_TYPE).unwrap().as_str()
        ));
        return;
    }

    if !check_content_len(&http_request) {
        respond(stream, &response::gen_response(
            status::PAYLOAD_TOO_LARGE,
            fs::read_to_string(json_msg_path::INVALID_CONTENT_LENGTH).unwrap().as_str()
        ));
        return;
    }

    let user = login_user(&http_request, data);
    if user.is_none() {
        respond(stream, &response::gen_response(
            status::ANAUTHORIZED,
            fs::read_to_string(json_msg_path::INVALID_CREDENTIALS).unwrap().as_str()
        ));
        return;
    }

    let request_line_splited = split_request_line(&http_request);
    if !check_http_standard(&request_line_splited) {
        respond(stream, &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::UNSUPORTED_STANDARD).unwrap().as_str()
        ));
        return;
    }

    let request_path = get_request_path(&request_line_splited);
    if request_path.len() < 1 { // GET /
        respond(stream, response::WELCOME_TXT);
        return;
    }

    let request_type_wrapped = get_request_type(&request_line_splited);

    if request_type_wrapped.is_err() {
        respond(stream, &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::UNSUPORTED_COMMAND).unwrap().as_str()
        ));
        return;
    }

    let request_type = request_type_wrapped.unwrap();

    match request_type {
        RequestType::Get => handle_get_request(stream, data, &request_path),

        _ => respond(stream, &response::gen_response(
            status::METHOD_NOT_ALLOWED,
            fs::read_to_string(json_msg_path::UNSUPORTED_COMMAND).unwrap().as_str()
        ))
    }
    return;
}

fn check_content_type(http_request: &Vec< String >) -> bool {
    let content_type_raw = http_request
        .iter()
        .find(|&s| s.starts_with("Content-Type: application/json"));
    content_type_raw.is_some()
}

fn check_content_len(http_request: &Vec< String >) -> bool {
    let content_len_str = http_request
        .iter()
        .find(|&s| s.starts_with("Content-Length: "));

    if content_len_str.is_none() {
        return false;
    }

    let content_len_raw = content_len_str
        .unwrap()
        .split_whitespace()
        .last();

    if content_len_raw.is_none() {
        return false;
    }

    let content_len_wrapped = content_len_raw
        .unwrap()
        .parse::< usize >();

    if content_len_wrapped.is_err() {
        return false;
    }

    static MAX_CONTENT_LEN: usize = 500;
    let content_len = content_len_wrapped.unwrap();

    content_len <= MAX_CONTENT_LEN
}

fn split_request_line(http_request: &Vec< String >) -> Vec<&str> {
    http_request
        .first()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
}

fn check_http_standard(request_line_splited: &Vec< &str >) -> bool {
    let http_standard = request_line_splited
        .last()
        .unwrap();

    **http_standard == *"HTTP/1.1"
}

fn get_request_type(request_line_splited: &Vec< &str >) -> Result< RequestType, ()> {
    let request_type = request_line_splited[0];
    RequestType::from_str(request_type)
}

fn get_request_path(request_line_splited: &Vec< &str >) -> Vec<String> {
    let request_path_preformated = request_line_splited[1..(request_line_splited.len()-1)]
        .join(" ");
    let request_path = request_path_preformated
        .split('/')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    return request_path;
}

fn login_user<'a>(http_request: &Vec< String >, data: &'a mut Data) -> Option< &'a mut User > {
    let credentials_wrapped = http_request
        .iter()
        .find(|&s| s.starts_with("Authorization: basic "));

    if credentials_wrapped.is_none() {
        return None;
    }

    let credentials = credentials_wrapped.unwrap();

    return data.login(credentials);
}

fn respond(mut stream: &TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_get_request(stream: &TcpStream, data: &Data, request_path: &Vec<String>) {
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
