use std::{collections::HashMap, option::Option, vec::Vec};

use crate::group::Group;
use crate::user::User;

pub struct Data {
    users: HashMap<String, User>,
    groups: HashMap<String, Group>,
}

impl Data {
    pub fn build() -> Self {
        Data {
            users: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn get_users(&self) -> Vec<(&String, &User)> {
        let result = self.users.iter().collect::<Vec<(&String, &User)>>();
        result
    }

    pub fn get_groups(&self) -> Vec<(&String, &Group)> {
        let result = self.groups.iter().collect::<Vec<(&String, &Group)>>();
        result
    }

    pub fn get_user(&self, login: &str) -> Option<&User> {
        return self.users.get(login);
    }

    pub fn get_user_mut(&mut self, login: &str) -> Option<&mut User> {
        return self.users.get_mut(login);
    }

    pub fn get_group(&self, id: &str) -> Option<&Group> {
        return self.groups.get(id);
    }

    pub fn get_group_mut(&mut self, id: &str) -> Option<&mut Group> {
        return self.groups.get_mut(id);
    }

    pub fn create_group(&mut self, id: &str, login: &str) {
        if self.groups.contains_key(id) {
            return;
        }
        self.groups.insert(id.to_string(), Group::new(login));
    }

    pub fn register_user(&mut self, login: &str, password: &str) {
        self.users.insert(login.to_string(), User::new(password));
    }

    pub fn checked_delete_group(&mut self, id: &str, login: &str) -> bool {
        let target_group_wrapped = self.get_group(id);
        if target_group_wrapped.is_none() {
            return false;
        }
        let target_group = target_group_wrapped.unwrap();
        if !target_group.get_admins().contains_key(&login.to_string()) {
            return false;
        }
        self.groups.remove(id);
        true
    }

    pub fn checked_delete_user(&mut self, id: &str, login: &str) -> bool {
        if id == login {
            return false;
        }
        self.delete_user(id)
    }

    fn delete_user(&mut self, id: &str) -> bool {
        let maybe_user = self.users.remove(id);
        maybe_user.is_some()
    }

    pub fn login(&self, credentials: &str) -> Option<(&String, &User)> {
        let credentials_as_whole = credentials.split_whitespace().collect::<Vec<_>>();
        let login = credentials_as_whole[2];
        let password = credentials_as_whole[3];
        let user_wrapped = self.users.get_key_value(login);
        user_wrapped.filter(|u| u.1.get_password() == password)
    }
}
