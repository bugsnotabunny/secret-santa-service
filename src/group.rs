use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use std::collections::HashMap;

use crate::engrouped_user::EngroupedUser;

#[derive(Serialize)]
pub struct Group {
    users: HashMap<String, EngroupedUser>,
}

impl Group {
    pub fn new(login: &str) -> Group {
        let mut result = Group {
            users: HashMap::new(),
        };
        result
            .users
            .insert(login.to_string(), EngroupedUser::new(true));
        result
    }

    pub fn make_admin(&mut self, id: &str, login: &str) -> bool {
        if !self.is_admin(login) {
            return false;
        }

        let user_wrapped = self.users.get_mut(id);
        if user_wrapped.is_none() {
            return false;
        }
        user_wrapped.unwrap().set_is_admin(true);
        true
    }

    pub fn is_admin(&self, id: &str) -> bool {
        let user_wrapped = self.users.get(id);
        if user_wrapped.is_none() {
            return false;
        }
        let user = user_wrapped.unwrap();
        user.get_is_admin()
    }

    pub fn entry(&mut self, login: &str) {
        let is_admin_ = false;
        let eng_user = EngroupedUser::new(is_admin_);
        self.users.insert(login.to_string(), eng_user);
    }

    pub fn exit(&mut self, login: &String) {
        let admins: HashMap<&String, &EngroupedUser> = self.get_admins();
        if admins.len() > 1 || !admins.contains_key(login) {
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
        admins
    }

    pub fn get_all_users(&self) -> &HashMap<String, EngroupedUser> {
        &self.users
    }

    pub fn shuffle_santas(&mut self) {
        if self.users.capacity() < 2 {
            return;
        }

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
