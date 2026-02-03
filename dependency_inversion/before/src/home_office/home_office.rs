use std::io::{Error, ErrorKind};
use std::sync::Once;
use crate::home_office::passport::passport::Passport;
use crate::home_office::passport::repository::PassportRepository;
use crate::home_office::visa::repository::VisaRepository;
use crate::home_office::visa::visa::Visa;

pub struct HomeOffice {}

impl HomeOffice {
    pub fn find_passport(&self, last_name: String, passport_number: u32) -> Result<Passport, Error> {
        let result = PassportRepository{ passports: vec![
            Passport{
                passport_number,
                last_name: last_name.clone(),
                first_name: "Adam".to_string(),
                middle_name: Some("George".to_string()),
                date_of_birth: "1988-09-10".to_string(),
            }
        ] }.find_one(passport_number, last_name);

        match result {
            Ok(Some(result)) => {
                println!("{:?}", result);
                Ok(result)
            },
            Ok(None) => Err(Error::new(ErrorKind::NotFound, "Passport not found")),
            Err(error) => Err(Error::new(ErrorKind::Other, error.to_string())),
        }
    }

    pub fn find_visa_application(
        &self,
        first_name: String,
        last_name: String,
        date_of_birth: String,
    ) -> Result<Visa, Error> {
        let result = VisaRepository{
            visas: vec![Visa{
                visa_type: "TIER_2".to_string(),
                expiration_date: "2029-09-10".to_string(),
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                date_of_birth: date_of_birth.clone(),
            }],
        }.find_one(first_name, last_name, date_of_birth);

        match result {
            Ok(Some(result)) => {
                println!("{:?}", result);
                Ok(result)
            },
            Ok(None) => Err(Error::new(ErrorKind::NotFound, "Visa application has not been found")),
            Err(error) => Err(Error::new(ErrorKind::Other, error.to_string())),
        }
    }
}
