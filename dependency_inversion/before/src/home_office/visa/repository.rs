use std::io::{Error, ErrorKind};
use crate::home_office::visa::visa::Visa;

pub struct VisaRepository {
    pub visas: Vec<Visa>,
}

impl VisaRepository {
    pub fn find_one(
        &self,
        first_name: String,
        last_name: String,
        date_of_birth: String,
    ) -> Result<Option<Visa>, Error> {
        // Using vector's built-in trait for iteration
        // Then using closure function to filter (pick) a record
        // let visa = self.visas.iter().find(|v|
        //     v.first_name == first_name
        //         && v.last_name == last_name
        //         && v.date_of_birth == date_of_birth
        // );
        // return Ok(Some(visa.unwrap().clone()));

        if self.visas.is_empty() {
            // This is a pretend error if there is an issue with database
            return Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
        }

        // Using rust's traditional loop to filter (pick) a record
        for visa in &self.visas {
            if visa.first_name == first_name
                && visa.last_name == last_name
                && visa.date_of_birth == date_of_birth
            {
                return Ok(Some(visa.clone()));
            }
        }

        Ok(None)
    }
}
