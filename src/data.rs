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

    pub fn get_user(&self, login: &String) -> Option<&User> {
        return self.users.get(login);
    }

    pub fn get_group(&self, id: &String) -> Option<&Group> {
        return self.groups.get(id);
    }

    // fn create_group(...);
    // fn register_user(...);

    pub fn delete_group(&mut self, id: &str) -> bool {
        let maybe_group = self.groups.remove(id);
        maybe_group.is_some()
    }

    pub fn delete_user(&mut self, id: &str) -> bool {
        let maybe_user = self.groups.remove(id);
        maybe_user.is_some()
    }

    pub fn login(&mut self, credentials: &str) -> Option<&mut User> {
        let credentials_as_whole = credentials.split_whitespace().collect::<Vec<_>>();
        let login = credentials_as_whole[2];
        let password = credentials_as_whole[3];
        let user_wrapped = self.users.get_mut(login);
        user_wrapped.filter(|u| u.get_password() == password)
    }
}
