// Debug is used for printing struct for debug with {:?} in println!
// Clone is used to allow us copy a return value of passport instead of re-creating it
#[derive(Debug,Clone)]
pub struct Visa {
    pub visa_type: String,
    pub expiration_date: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}
