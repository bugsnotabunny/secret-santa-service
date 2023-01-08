use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
    str::FromStr,
};

use crate::{
    data::Data,
    mucho_texto::{response, status},
    request_type::RequestType,
    respond::*,
    user::User,
};

pub fn handle_connection(mut stream: &TcpStream, data: &mut Data) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let request_line_splited = split_request_line(&http_request);
    if !check_http_standard(&request_line_splited) {
        respond_unsuported_standard(stream);
        return;
    }

    let request_path = get_request_path(&request_line_splited);
    if request_path.is_empty() {
        // GET /
        respond_welcome(stream);
        return;
    }

    let request_type_wrapped = get_request_type(&request_line_splited);
    if request_type_wrapped.is_err() {
        respond_unsuported_command(stream);
        return;
    }

    let request_type = request_type_wrapped.unwrap();
    match request_type {
        RequestType::Get => handle_get_request(stream, data, &request_path),
        RequestType::Post => handle_post_request(),
        RequestType::Delete => handle_delete_request(stream, data, &request_path, &http_request),
    }
}

fn split_request_line(http_request: &[String]) -> Vec<&str> {
    http_request
        .first()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
}

fn check_http_standard(request_line_splited: &[&str]) -> bool {
    let http_standard = request_line_splited.last().unwrap();

    **http_standard == *"HTTP/1.1"
}

fn get_request_type(request_line_splited: &[&str]) -> Result<RequestType, ()> {
    let request_type = request_line_splited[0];
    RequestType::from_str(request_type)
}

fn get_request_path(request_line_splited: &[&str]) -> Vec<String> {
    let request_path_preformated =
        request_line_splited[1..(request_line_splited.len() - 1)].join(" ");
    let request_path = request_path_preformated
        .split('/')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    request_path
}

fn login_user<'a>(http_request: &[String], data: &'a mut Data) -> Option<&'a mut User> {
    let credentials_wrapped = http_request
        .iter()
        .find(|&s| s.starts_with("Authorization: basic "));
    let credentials = credentials_wrapped?;
    data.login(credentials)
}

fn handle_get_request(stream: &TcpStream, data: &mut Data, request_path: &Vec<String>) {
    if request_path[0] == "users" {
        if request_path.len() < 2 {
            let users = data.get_users();
            let json_users = serde_json::to_string(&users).unwrap();
            respond(
                stream,
                response::gen_response(status::OK, json_users.as_str()).as_str(),
            );
            return;
        }

        let user_wrapped = data.get_user(&request_path[1].to_string());
        if let Some(user) = user_wrapped {
            let json_user = serde_json::to_string(user).unwrap();
            respond(
                stream,
                response::gen_response(status::OK, json_user.as_str()).as_str(),
            );
        }
    } else if request_path[0] == "groups" {
        if request_path.len() < 2 {
            let groups = data.get_groups();
            let json_groups = serde_json::to_string(&groups).unwrap();
            respond(
                stream,
                response::gen_response(status::OK, json_groups.as_str()).as_str(),
            );
            return;
        }

        let group_wrapped = data.get_group(&request_path[1].to_string());
        if let Some(user) = group_wrapped {
            let json_group = serde_json::to_string(user).unwrap();
            respond(
                stream,
                response::gen_response(status::OK, json_group.as_str()).as_str(),
            );
        }
    }
    respond_not_found(stream)
}

fn handle_post_request() {
    
}

fn handle_delete_request(
    stream: &TcpStream,
    data: &mut Data,
    request_path: &[String],
    http_request: &[String],
) {
    let user = login_user(http_request, data);
    if user.is_none() {
        respond_invalid_credentials(stream);
        return;
    }

    if request_path.len() < 2 {
        respond_unsuported_command(stream);
        return;
    }

    let was_deleted = if request_path[0] == "users" {
        data.delete_user(&request_path[2])
    } else if request_path[0] == "groups" {
        data.delete_group(&request_path[2])
    } else {
        false
    };

    if was_deleted {
        respond_deleted_successfully(stream)
    } else {
        respond_not_found(stream);
    }
}
