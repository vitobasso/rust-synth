use std::time::Duration;

//TODO replace with std Duration when stable

pub fn div_duration(a: Duration, b: Duration) -> f64 {
    a.as_secs_f64() / b.as_secs_f64()
}
