use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EngroupedUser {
    permission: bool,
    login: String,
    receivers_login: String
}
impl EngroupedUser {
    pub fn get_permission(self) -> bool{
        self.permission
    }

    pub fn get_login(&self) -> &String{
        &self.login
    }

    pub fn get_recievers_login(&self) -> &String{
        &self.receivers_login
    }
}