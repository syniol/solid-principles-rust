use std::io::{Error};
use crate::payment::providers::provider::{DebitPayment, PaymentProviderName};
use crate::user::member::Member;

pub struct OpenBanking {}

pub const PAYMENT_OB: PaymentProviderName = "OB";

impl DebitPayment for OpenBanking {
    fn pay(&self, user: &Member) -> Result<(), Error> {
        if user.preferred_payment_method != PAYMENT_OB {
            return Ok(())
        }

        println!("Paying member {} with an id {} Open Balance with OB",
                 user.name,
                 user.id,
        );

        Ok(())
    }
}
