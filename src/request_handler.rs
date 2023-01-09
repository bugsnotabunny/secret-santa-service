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
        RequestType::Get => handle_get_request(stream, data, &request_path, &http_request),
        RequestType::Post => handle_post_request(stream, data, &request_path, &http_request),
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

fn login_user<'a>(http_request: &[String], data: &'a Data) -> Option<(&'a String, &'a User)> {
    let credentials_wrapped = http_request
        .iter()
        .find(|&s| s.starts_with("Authorization: basic "));
    let credentials = credentials_wrapped?;
    data.login(credentials)
}

fn handle_get_request(
    stream: &TcpStream,
    data: &mut Data,
    request_path: &[String],
    http_request: &[String],
) {
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
        if let Some(group) = group_wrapped {
            if request_path.len() > 2 {
                if request_path[2] == "santafor" {
                    let user_wrapped = login_user(http_request, data);
                    if user_wrapped.is_none() {
                        respond_invalid_credentials(stream);
                        return;
                    }
                    let login = user_wrapped.unwrap().0.clone();
                    let eng_user = group.get_all_users().get(login.as_str());
                    if eng_user.is_none() {
                        respond_not_found(stream);
                        return;
                    }
                    let recievers_login = eng_user.unwrap().get_recievers_login();
                    let recievers_login_json =
                        "{\"santa_for\":\"".to_string() + recievers_login + "\"}";
                    respond(
                        stream,
                        &response::gen_response(status::OK, recievers_login_json.as_str()),
                    );
                    return;
                }
                respond_not_found(stream);
                return;
            }
            let json_group = serde_json::to_string(group).unwrap();
            respond(
                stream,
                response::gen_response(status::OK, json_group.as_str()).as_str(),
            );
        }
    }
    respond_not_found(stream)
}

fn handle_post_request(
    stream: &TcpStream,
    data: &mut Data,
    request_path: &[String],
    http_request: &[String],
) {
    if request_path[0] == "register" {
        let credentials_wrapped = http_request
            .iter()
            .find(|&s| s.starts_with("Authorization: basic "));
        let credentials = credentials_wrapped.unwrap();
        let cred_splited = credentials.split_whitespace().collect::<Vec<_>>();
        let login = cred_splited[2];
        let password = cred_splited[3];
        data.register_user(login, password);
        respond_registered_successfully(stream);
        return;
    }

    let user_wrapped = login_user(http_request, data);
    if user_wrapped.is_none() {
        respond_invalid_credentials(stream);
        return;
    }
    let login = user_wrapped.unwrap().0.clone();

    if request_path.len() < 2 {
        respond_unsuported_command(stream);
        return;
    }

    if request_path[0] == "groups" {
        let group_wrapped = data.get_group_mut(request_path[1].as_str());
        if group_wrapped.is_none() {
            respond_not_found(stream);
            return;
        }
        let group = group_wrapped.unwrap();
        if request_path[2] == "makeadmin" {
            let status = group.make_admin(request_path[3].as_str(), login.as_str());
            if !status {
                respond_registered_successfully(stream);
            } else {
                respond_not_allowed(stream);
            }
        } else if request_path[2] == "assignsantas" {
            group.shuffle_santas();
        }
        return;
    }
    let group_wrapped = data.get_group_mut(request_path[1].as_str());
    if group_wrapped.is_none() {
        if request_path[0] == "join" {
            data.create_group(request_path[1].as_str(), login.as_str());
            respond_registered_successfully(stream);
        } else {
            respond_not_found(stream);
        }
        return;
    }
    let group = group_wrapped.unwrap();

    if request_path[0] == "join" {
        group.entry(&login);
    } else if request_path[0] == "leave" {
        group.exit(&login);
    } else {
        respond_unsuported_command(stream);
    }
}

fn handle_delete_request(
    stream: &TcpStream,
    data: &mut Data,
    request_path: &[String],
    http_request: &[String],
) {
    let user_wrapped = login_user(http_request, data);
    if user_wrapped.is_none() {
        respond_invalid_credentials(stream);
        return;
    }
    let login = user_wrapped.unwrap().0.clone();

    if request_path.len() != 2 {
        respond_unsuported_command(stream);
        return;
    }

    let was_deleted = if request_path[0] == "users" {
        data.checked_delete_user(&request_path[1], login.as_str())
    } else if request_path[0] == "groups" {
        data.checked_delete_group(&request_path[1], login.as_str())
    } else {
        false
    };

    if was_deleted {
        respond_deleted_successfully(stream)
    } else {
        respond_not_found(stream);
    }
}
