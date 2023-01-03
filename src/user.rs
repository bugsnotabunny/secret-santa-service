// use serde::{Deserialize, Serialize};
// use serde_json::Result;

// #[derive(Serialize, Deserialize)]
pub struct User {
    password: String
}

impl User {
    pub fn get_password(&self) -> &String {
        return &self.password;
    }
}