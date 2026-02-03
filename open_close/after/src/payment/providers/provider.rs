use std::io::{Error, ErrorKind};
use crate::user::member::Member;

pub type PaymentProviderName = &'static str;

pub struct DebitPaymentProvider {
    pub providers: Vec<Box<dyn DebitPayment>>,
}

pub trait DebitPayment {
    fn pay(&self, user: &Member) -> Result<(), Error>;
}

impl DebitPaymentProvider {
    pub fn pay(&self, member: &Member) -> Result<(), Error> {
        for payment_provider in &self.providers {
            let resp = payment_provider.pay(member);
            if resp.is_err() {
                return Err(resp.err().unwrap());
            }
        }

        Err(Error::new(
            ErrorKind::Unsupported,
            "unsupported payment method",
        ))
    }
}
