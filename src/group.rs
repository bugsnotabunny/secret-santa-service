use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engrouped_user::EngroupedUser;

#[derive(Serialize, Deserialize)]

pub struct Group {
    users_hashmap: HashMap<String, EngroupedUser>,
}

impl Group{
    pub fn new() -> Group{
        Group {users_hashmap: HashMap::new()}
    }

    pub fn entry(&mut self, eng_user: EngroupedUser){
        let login: &String = eng_user.get_login();
        self.users_hashmap.insert(login.to_string(), eng_user);
    }
    
    fn exit(&mut self, eng_user: EngroupedUser){
        let login: &String = eng_user.get_login();
        let opt: Option<EngroupedUser> = self.users_hashmap.remove(login);
    }
}
