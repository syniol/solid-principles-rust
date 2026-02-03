use std::io::{Error, ErrorKind};
use crate::payment::providers::open_banking::{OpenBanking, PAYMENT_OB};
use crate::payment::providers::stripe::{Strip, PAYMENT_STRIP};
use crate::user::member::Member;

pub type PaymentProviderName = &'static str;

pub struct DebitPaymentProvider {
    pub stripe: Strip,
    pub open_banking: OpenBanking,
}

impl DebitPaymentProvider {
    pub fn pay(&self, member: &Member) -> Result<(), Error> {
        if member.preferred_payment_method == PAYMENT_STRIP {
            return self.stripe.process_payment(member)
        }

        if member.preferred_payment_method == PAYMENT_OB {
            return self.open_banking.pay_direct(member)
        }

        Err(Error::new(
            ErrorKind::Unsupported,
            "unsupported payment method",
        ))
    }
}
