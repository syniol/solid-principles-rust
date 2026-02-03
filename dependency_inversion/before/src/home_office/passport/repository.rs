use std::io::{Error, ErrorKind};
use crate::home_office::passport::passport::Passport;

pub struct PassportRepository {
    pub passports: Vec<Passport>,
}

impl PassportRepository {
    pub fn find_one(&self, passport_number: u32, last_name: String) -> Result<Option<Passport>, Error> {
        // Using vector's built-in trait for iteration
        // Then using closure function to filter (pick) a record
        // let passport = self.passports.iter().find(|p| p.passport_number == passport_number && p.last_name == last_name);
        // return Ok(Some(passport.unwrap().clone()));

        if self.passports.is_empty() {
            // This is a pretend error if there is an issue with database
            return Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
        }

        // Using rust's traditional loop to filter (pick) a record
        for passport in &self.passports {
            if passport.passport_number == passport_number && passport.last_name == last_name {
                return Ok(Some(passport.clone()));
            }

        }

        Ok(None)
    }
}
