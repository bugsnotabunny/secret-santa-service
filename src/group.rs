use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::engrouped_user::EngroupedUser;

#[derive(Serialize, Deserialize)]

pub struct Group {
    users: HashMap<String, EngroupedUser>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            users: HashMap::new(),
        }
    }

    pub fn entry(&mut self, login: &str) {
        let is_admin_ = self.users.is_empty();
        let eng_user = EngroupedUser::new(is_admin_);
        self.users.insert(login.to_string(), eng_user);
    }

    pub fn exit(&mut self, login: &String) {
        let admins: HashMap<&String, &EngroupedUser> = self.get_admins();
        if !admins.contains_key(login) {
            self.users.remove(login);
        }
    }

    pub fn get_admins(&self) -> HashMap<&String, &EngroupedUser> {
        let mut admins = HashMap::new();
        for (key, value) in &self.users {
            let is_admin: bool = value.get_is_admin();
            if is_admin {
                admins.insert(key, value);
            }
        }
        return admins;
    }

    pub fn get_all_users(&self) -> &HashMap<String, EngroupedUser> {
        return &self.users;
    }

    pub fn shuffle_santas(&mut self) {
        if self.users.capacity() > 1 {
            let mut users_vec = Vec::new();
            for login in self.users.keys() {
                users_vec.push(login.clone());
            }
            users_vec.shuffle(&mut thread_rng());
            for (login, user) in self.users.iter_mut() {
                if users_vec[0] != *login {
                    user.set_receiver(&users_vec[0]);
                    users_vec.remove(0);
                } else {
                    user.set_receiver(&users_vec[1]);
                    users_vec.remove(1);
                }
            }
        }
    }
}
