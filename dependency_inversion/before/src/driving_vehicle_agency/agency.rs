// use std::io::{Error, ErrorKind};
// use crate::driving_vehicle_agency::person::person::Person;
// use crate::driving_vehicle_agency::person::repository::PersonRepository;
// use crate::home_office::home_office::HomeOffice;

// pub struct DrivingVehicleAgency {}

// impl DrivingVehicleAgency {
//     pub fn validate_residency_status(
//         license_number: String,
//     ) -> Result<bool, Error> {
//         let person_repository = &PersonRepository{
//             persons: vec![],
//         };
//
//         let person = person_repository.find_one(license_number);
//         if person.is_err() {
//             return Err(Error::new(ErrorKind::NotFound, "HomeOffice"));
//         }
//
//         if person.is_ok() {
//             let home_office = HomeOffice {};
//             let found_person = person?.unwrap();
//
//             if found_person.passport_number.is_some() {
//                 let passport = home_office.find_passport(
//                     found_person.last_name.to_string(),
//                     found_person.passport_number.unwrap(),
//                 );
//                 if passport.is_err() {
//                     return Err(Error::new(ErrorKind::NotFound, passport.err().unwrap()));
//                 }
//
//                 if passport.is_ok() {
//                     return Ok(true);
//                 } else {
//                     let visa_application = home_office.find_visa_application(
//                         found_person.first_name.to_string(),
//                         found_person.last_name.to_string(),
//                         found_person.date_of_birth.to_string(),
//                     );
//
//                     if visa_application.is_err() {
//                         return Err(Error::new(ErrorKind::NotFound, visa_application.err().unwrap()));
//                     }
//
//                     if visa_application?.is_some() {
//                         return Ok(true);
//                     }
//                 }
//             }
//         }
//
//         Ok(false)
//     }
// }

