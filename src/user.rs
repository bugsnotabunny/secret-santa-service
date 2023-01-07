use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    login: String,
    password: String,
    nickname: String,
}

impl User {
    pub fn get_login(&self) -> &String {
        return &self.login;
    }
    pub fn get_password(&self) -> &String {
        return &self.password;
    }
    pub fn get_nickname(&self) -> &String {
        return &self.nickname;
    }
}
