use std::{collections::HashMap, option::Option, vec::Vec};

use crate::group::Group;
use crate::user::User;

pub struct Data {
    users: HashMap<String, User>,
    groups: HashMap<String, Group>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            users: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    pub fn get_users(&self) -> Vec<(&String, &User)> {
        let result = self.users.iter().collect::<Vec<(&String, &User)>>();
        return result;
    }

    pub fn get_groups(&self) -> Vec<(&String, &Group)> {
        let result = self.groups.iter().collect::<Vec<(&String, &Group)>>();
        return result;
    }

    pub fn get_user(&self, login: &String) -> Option<&User> {
        return self.users.get(login);
    }

    pub fn get_group(&self, id: &String) -> Option<&Group> {
        return self.groups.get(id);
    }

    // fn create_group(...);
    // fn register_user(...);
    // fn delete_group(...);
    // fn delete_user(...);

    pub fn login(&mut self, credentials: &str) -> Option<&mut User> {
        let credentials_as_whole = credentials.split_whitespace().collect::<Vec<_>>();
        let login = credentials_as_whole[2];
        let password = credentials_as_whole[3];
        let user_wrapped = self.users.get_mut(login);
        match user_wrapped {
            Some(user) => {
                if user.get_password() == password {
                    Some(user)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
