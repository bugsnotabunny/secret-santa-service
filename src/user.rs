use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct User {
    password: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = serializer.serialize_struct("User", 0)?;
        s.end()
    }
}

impl User {
    pub fn new(password: &str) -> Self {
        User {
            password: password.to_string(),
        }
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }
}
