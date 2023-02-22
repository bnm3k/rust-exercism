fn error_rate(speed: u8) -> Option<f64> {
    if speed >= 1 && speed <= 4 {
        Some(1.0)
    } else if speed >= 5 && speed <= 8 {
        Some(0.9)
    } else if speed >= 9 && speed <= 10 {
        Some(0.77)
    } else {
        None
    }
}
pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed == 0 {
        0.0
    } else {
        (speed as f64)
            * (221 as f64)
            * error_rate(speed)
                .expect(format!("speed: {speed} should be between 1 and 10").as_str())
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let per_hour = production_rate_per_hour(speed);
    let per_minute = (per_hour / 60.0) as u32;
    return per_minute;
}
