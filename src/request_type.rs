use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum RequestType {
    Get,
    Post,
}

impl FromStr for RequestType {
    type Err = ();
    fn from_str(input: &str) -> Result<RequestType, Self::Err> {
        match input {
            "GET" => Ok(RequestType::Get),
            "POST" => Ok(RequestType::Post),
            _ => Err(()),
        }
    }
}
