use std::time::Duration;
use std::u64;

//TODO replace with std Duration after rust 1.33

const NANOS_PER_SEC: u32 = 1_000_000_000;
const MAX_NANOS_F64: f64 = ((u64::MAX as u128 + 1)*(NANOS_PER_SEC as u128)) as f64;

pub fn as_nanos(duration: Duration) -> u64 {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();
    secs * 1_000_000 + u64::from(nanos)
}

pub fn as_millis(duration: Duration) -> u64{
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    secs * 1_000 + u64::from(millis)
}

pub fn as_float_secs(duration: Duration) -> f64 {
    (duration.as_secs() as f64) + (duration.subsec_nanos() as f64) / (NANOS_PER_SEC as f64)
}

pub fn from_float_secs(secs: f64) -> Duration {
    let nanos =  secs * (NANOS_PER_SEC as f64);
    if !nanos.is_finite() {
        panic!("got non-finite value when converting float to duration");
    }
    if nanos >= MAX_NANOS_F64 {
        panic!("overflow when converting float to duration");
    }
    if nanos < 0.0 {
        panic!("underflow when converting float to duration");
    }
    let nanos =  nanos as u128;
    let secs = (nanos / (NANOS_PER_SEC as u128)) as u64;
    let subsec_nanos = (nanos % (NANOS_PER_SEC as u128)) as u32;
    Duration::new(secs, subsec_nanos)
}

pub fn div_duration(a: Duration, b: Duration) -> f64 {
    as_float_secs(a) / as_float_secs(b)
}

pub fn mul_f64(duration: Duration, float: f64) -> Duration {
    from_float_secs(float * as_float_secs(duration))
}
