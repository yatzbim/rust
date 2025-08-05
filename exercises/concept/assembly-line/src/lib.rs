pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = if speed < 1 {
        0.0
    } else if speed < 5 {
        1.0
    } else if speed < 9 {
        0.9
    } else if speed < 11 {
        0.77
    } else {
        panic!("{speed} is greater than maximum allowed speed of 10!")
    };

    rate * speed as f64 * 221.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate = production_rate_per_hour(speed);
    let items_per_minute = rate / 60.0;
    items_per_minute as u32
}
