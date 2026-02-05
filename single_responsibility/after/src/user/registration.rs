use crate::user::user::User;
use std::io::{Error, ErrorKind};
use crate::util::date_time::DateTime;

pub struct Registration {
    date_time: DateTime,
}

pub fn new() -> Registration {
    Registration {
        date_time: DateTime{}
    }
}

static MINIMUM_AGE_LIMIT_IN_YEARS: i16 = 18;

impl Registration {
    pub fn register_user(&self, user: &User) -> Result<String, Error> {
        let parsed_date = self.date_time.parse_raw_date(user.date_of_birth);
        let span = self.date_time.get_date_span_in_years(parsed_date.unwrap(), Zoned::now().date());


        Ok(format!("Successfully registered user: {}", user.name).to_string())
    }
}

#[test]
fn test_register_user_ok() {
    use jiff::ToSpan;

    let res = new().register_user(&User {
        date_of_birth: Zoned::now()
            .checked_sub(MINIMUM_AGE_LIMIT_IN_YEARS.years())
            .unwrap()
            .date()
            .to_string()
            .as_str(),
        name: "Test User",
    });

    match res {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }

    assert_eq!(res.unwrap(), "Successfully registered user: Test User");
}

#[test]
fn test_register_user_age_error() {
    use jiff::ToSpan;

    let res = new().register_user(&User {
        date_of_birth: Zoned::now()
            .checked_sub((MINIMUM_AGE_LIMIT_IN_YEARS - 1).years())
            .unwrap()
            .date()
            .to_string()
            .as_str(),
        name: "Test User",
    });

    match res {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e.to_string(), "you need to be at least 18 years"),
    }
}

#[test]
fn test_register_user_age_parse_error() {
    let res = new().register_user(&User {
        date_of_birth: "7612362178",
        name: "Test User",
    });

    match res {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(
            e.to_string(),
            "unexpected error parsing date of birth. Date format must be iso-8601",
        ),
    }
}
