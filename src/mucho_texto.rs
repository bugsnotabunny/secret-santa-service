pub mod status {
    pub static PAYLOAD_TOO_LARGE: &str = "HTTP/1.1 413 Payload Too Large";
    pub static METHOD_NOT_ALLOWED: &str = "HTTP/1.1 405 Method Not Allowed";
    pub static NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
    pub static ANAUTHORIZED: &str = "HTTP/1.1 401 Unauthorized";
    pub static OK: &str = "HTTP/1.1 200 OK";
    pub static NO_CONTENT: &str = "HTTP/1.1 204 No Content";
}

pub mod json_msg_path {
    pub static INVALID_CONTENT_LENGTH: &str = "answers/INVALID_CONTENT_LENGTH.json";
    pub static INVALID_CONTENT_TYPE: &str = "answers/INVALID_CONTENT_TYPE.json";
    pub static INVALID_CREDENTIALS: &str = "answers/INVALID_CREDENTIALS.json";
    pub static NO_CONTENT: &str = "answers/NO_CONTENT.json";
    pub static UNSUPORTED_STANDARD: &str = "answers/UNSUPORTED_STANDARD.json";
    pub static UNSUPORTED_COMMAND: &str = "answers/UNSUPORTED_COMMAND.json";
    pub static DELETED_SUCCESSFULLY: &str = "answers/DELETED_SUCCESSFULLY.json";
    pub static WELCOME: &str = "answers/WELCOME.json";
}

pub mod response {
    pub fn gen_response(status: &str, json_message: &str) -> String {
        let len = json_message.len();
        format!("{status}\r\n{DEFAULT_RESPONSE_HEADERS}{len}\r\n\r\n{json_message}")
    }

    static DEFAULT_RESPONSE_HEADERS: &str = "Content-Type: application/json\r\nContent-Length: ";
}
