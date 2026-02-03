// Debug is used for printing struct for debug with {:?} in println!
// Clone is used to allow us copy a return value of passport instead of re-creating it
#[derive(Debug,Clone)]
pub struct Passport {
    pub passport_number: u32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub date_of_birth: String,
}
