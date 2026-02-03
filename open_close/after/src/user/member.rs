use crate::payment::providers::provider::PaymentProviderName;

pub struct Member {
    pub id: usize,
    pub name: &'static str,
    pub preferred_payment_method: PaymentProviderName,
}
