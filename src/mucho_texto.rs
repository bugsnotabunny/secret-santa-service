pub mod status {
    pub static PAYLOAD_TOO_LARGE: &str = "HTTP/1.1 413 Payload Too Large";
    pub static METHOD_NOT_ALLOWED: &str = "HTTP/1.1 405 Method Not Allowed";
    pub static NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
    pub static ANAUTHORIZED: &str = "HTTP/1.1 401 Unauthorized";
    pub static OK: &str = "HTTP/1.1 200 OK";
}

pub mod response {
    use super::status;

    pub static INVALID_CONTENT: &str = status::NOT_FOUND;
    pub static INVALID_CREDENTIALS: &str = status::ANAUTHORIZED;
    pub static UNSUPORTED_STANDARD: &str = status::METHOD_NOT_ALLOWED;
    pub static UNSUPORTED_REQUEST: &str = status::METHOD_NOT_ALLOWED;
    pub static WELCOME_TXT: &str = WELCOME_MESSAGE;

    pub fn gen_response(status: &str, json_message: &str) -> String {
        let len = json_message.len();
        format!("{status}\r\nContent-Lenqgth: {len}\r\n\r\n{json_message}")
    }

    static WELCOME_MESSAGE: &str = ""; // must be in json format
}
