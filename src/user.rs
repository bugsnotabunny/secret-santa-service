use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    login: String,
    password: String,
    nickname: String,
}

impl User {
    pub fn get_login(&self) -> &String {
        &self.login
    }
    pub fn get_password(&self) -> &String {
        &self.password
    }
    pub fn get_nickname(&self) -> &String {
        &self.nickname
    }
}
