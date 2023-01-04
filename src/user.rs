use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    password: String,
}

impl User {
    pub fn get_password(&self) -> &String {
        &self.password
    }
}
