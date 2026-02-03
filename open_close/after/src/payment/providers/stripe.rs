use std::io::{Error};
use crate::payment::providers::provider::{DebitPayment, PaymentProviderName};
use crate::user::member::Member;

pub struct Strip {}

pub const PAYMENT_STRIP: PaymentProviderName = "Stripe";

impl DebitPayment for Strip {
    fn pay(&self, user: &Member) -> Result<(), Error> {
        if user.preferred_payment_method != PAYMENT_STRIP {
            return Ok(())
        }

        println!("Paying member {} with an id {} Open Balance with Strip",
                 user.name,
                 user.id,
        );

        Ok(())
    }
}
