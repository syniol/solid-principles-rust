use crate::user::user::User;

const MINIMUM_AGE_IN_YEARS: i8 = 18;
pub struct AgeEligibility {}

impl AgeEligibility {
    fn check_age(&self, date_of_birth: &str) -> bool {
        true
    }

    pub fn check_user_age_eligibility(&self, user: &User) -> bool {
        self.check_age(user.date_of_birth)
    }
}
