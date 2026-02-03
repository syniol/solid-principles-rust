// use std::io::{Error, ErrorKind};
// use crate::driving_vehicle_agency::person::person::Person;
//
// pub struct PersonRepository {
//     pub persons: Vec<Person>,
// }
//
// impl PersonRepository {
//     pub fn find_one(&self, license_number: String) -> Result<Option<&Person>, Error> {
//         let found_person = self.persons.iter().find(|p| p.license_number == license_number);
//         if found_person.is_some() {
//             return Ok(found_person)
//         }
//
//         Err(Error::new(ErrorKind::UnexpectedEof, "Unexpected EOF"))
//     }
// }
