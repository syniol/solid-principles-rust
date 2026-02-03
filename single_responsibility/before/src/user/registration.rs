use std::io::{Error, ErrorKind};
use crate::user::user::User;

pub struct Registration {}

pub fn new() -> Registration {
    Registration{}
}

impl Registration {
    pub fn register_user(&self, user: &User) -> Result<String, Error> {
        if user.date_of_birth == "" {
            return Err(std::io::Error::new(
                ErrorKind::Unsupported,
                "unsupported age",
            ))
        }

        Ok(format!("Registering user {}", user.name).to_string())
    }
}
