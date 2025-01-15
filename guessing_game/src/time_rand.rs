use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_random_number(range: u64) -> u64 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    (time % range as u128) as u64
}
