const CARS_PER_HOUR_SPEED_UNIT: f64 = 221.0;
const CARS_PER_MINUTE_SPEED_UNIT: f64 = CARS_PER_HOUR_SPEED_UNIT / 60.0;

fn success_by_speed(speed: u8) -> f64 {
    let success_rate = if speed == 0 {
        0.0
    } else if speed <= 4 {
        1.0
    } else if speed <= 8 {
        0.9
    } else if speed <= 10 {
        0.77
    } else {
        panic!("Speed must be 0 through 10 inclusive!")
    };
    success_rate
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = success_by_speed(speed);
    success_rate * CARS_PER_HOUR_SPEED_UNIT * speed as f64
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let success_rate = success_by_speed(speed);
    let working_cars_per_minute = CARS_PER_MINUTE_SPEED_UNIT * success_rate * speed as f64;
    working_cars_per_minute as u32
}
