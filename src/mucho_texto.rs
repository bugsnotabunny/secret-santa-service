pub mod status {
    pub static PAYLOAD_TOO_LARGE: &str = "HTTP/1.1 413 Payload Too Large";
    pub static METHOD_NOT_ALLOWED: &str = "HTTP/1.1 405 Method Not Allowed";
    pub static NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
    pub static ANAUTHORIZED: &str = "HTTP/1.1 401 Unauthorized";
    pub static OK: &str = "HTTP/1.1 200 OK";
    pub static NO_CONTENT: &str = "HTTP/1.1 204 No Content";
}



pub mod json_msg_path {
    pub static INVALID_CONTENT_LENGTH: &str = "INVALID_CONTENT_LENGTH.json";
    pub static INVALID_CONTENT_TYPE: &str = "INVALID_CONTENT_TYPE.json";
    pub static INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS.json";
    pub static NO_CONTENT: &str = "NO_CONTENT.json";
}

pub mod response {
    pub static WELCOME_TXT: &str = "WELCOME_MESSAGE"; //as json

    pub fn gen_response(status: &str, json_message: &str) -> String {
        let len = json_message.len();
        format!("{status}\r\n{DEFAULT_RESPONSE_HEADERS}{len}\r\n\r\n{json_message}")
    }

    static DEFAULT_RESPONSE_HEADERS: &str = "Content-Type: application/json\r\nContent-Length: ";
}
