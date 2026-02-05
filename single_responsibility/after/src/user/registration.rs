use crate::user::user::User;
use jiff::civil::DateTime;
use jiff::{Error as JiffError, Unit, Zoned};
use std::io::{Error, ErrorKind};

pub struct Registration {}

pub fn new() -> Registration {
    Registration {}
}

static MINIMUM_AGE_LIMIT_IN_YEARS: i16 = 18;

impl Registration {
    pub fn validate_age(&self, dob: &str) -> Result<(), Error> {
        let date_of_birth: Result<DateTime, JiffError> = dob.parse();
        if date_of_birth.is_err() {
            eprintln!("{}", date_of_birth.unwrap_err().to_string());
            return Err(Error::new(
                ErrorKind::Unsupported,
                "unexpected error parsing date of birth. Date format must be iso-8601",
            ));
        }

        let date_span = date_of_birth
            .unwrap()
            .date()
            .until((Unit::Year, Zoned::now().date()));
        if date_span.is_err() {
            eprintln!("{}", date_span.unwrap_err().to_string());

            return Err(Error::new(
                ErrorKind::Unsupported,
                "unexpected error finding span for date of birth",
            ));
        }

        if date_span.unwrap().get_years() < MINIMUM_AGE_LIMIT_IN_YEARS {
            return Err(std::io::Error::new(
                ErrorKind::Unsupported,
                format!("you need to be at least {} years", MINIMUM_AGE_LIMIT_IN_YEARS),
            ));
        }

        return Ok(());
    }

    pub fn register_user(&self, user: &User) -> Result<String, Error> {
        let is_age_valid = self.validate_age(user.date_of_birth);
        if is_age_valid.is_err() {
            return Err(is_age_valid.unwrap_err());
        }

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
