use std::io::{Error, ErrorKind};
use crate::user::age_eligibility::{AgeEligibility};
use crate::user::user::User;

pub struct Registration {
    age_eligibility_service: AgeEligibility,
}

pub fn new() -> Registration {
    Registration{
        age_eligibility_service: AgeEligibility{}
    }
}

impl Registration {
    pub fn register_user(&self, user: &User) -> Result<String, Error> {
        let is_eligible = self.age_eligibility_service.check_user_age_eligibility(user);
        if !is_eligible {
            return Err(std::io::Error::new(
                ErrorKind::Unsupported,
                "unsupported age",
            ))
        }

        Ok(format!("Registering user {}", user.name).to_string())
    }
}
