pub fn expected_minutes_in_oven() -> i32 {
    40
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    std::cmp::max(expected_minutes_in_oven() - actual_minutes_in_oven, 0)
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    std::cmp::max(number_of_layers * 2, 0)
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let number_of_layers = std::cmp::max(number_of_layers, 0);
    let actual_minutes_in_oven = std::cmp::max(actual_minutes_in_oven, 0);
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
