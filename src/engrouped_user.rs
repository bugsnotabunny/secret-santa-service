use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EngroupedUser {
    is_admin: bool,
    receivers_login: String
}
impl EngroupedUser {
    pub fn new(is_admin_: bool) -> EngroupedUser{
        let recivers_login_ = String::from("");
        EngroupedUser {is_admin: is_admin_, receivers_login: recivers_login_}
    }
    pub fn get_is_admin(&self) -> bool{
        self.is_admin
    }

    pub fn get_recievers_login(&self) -> &String{
        &self.receivers_login
    }

    pub fn set_is_admin(&mut self, is_admin_: bool){
        self.is_admin = is_admin_;
    }
}