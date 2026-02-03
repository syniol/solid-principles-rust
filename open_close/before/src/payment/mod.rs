pub mod providers;

use crate::payment::providers::provider::DebitPaymentProvider;
use crate::payment::providers::open_banking::OpenBanking;
use crate::payment::providers::stripe::Strip;

pub fn new() -> DebitPaymentProvider {
    DebitPaymentProvider{
        stripe: Strip {},
        open_banking: OpenBanking {},
    }
}
