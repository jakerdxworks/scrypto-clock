use scrypto::prelude::*;

pub fn get_current_time() -> u64 {
    Clock::current_time(TimePrecisionV2::Second)
    .seconds_since_unix_epoch
    .to_u64()
    .expect("msg")
}