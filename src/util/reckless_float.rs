use std::{hash, mem, cmp::Ordering};

/// Compares f64's for equality by binary value, ignoring that there are 16 million different NaN values.
/// Orders f64's considering NaN to be greater than everything, with undefined order between NaN's.
///
/// https://stackoverflow.com/questions/39638363/how-can-i-use-a-hashmap-with-f64-as-key-in-rust
/// https://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats
#[derive(Debug, Copy, Clone, PartialOrd)]
pub struct RecklessFloat(pub f64);

impl RecklessFloat {
    fn key(&self) -> u64 {
        unsafe { mem::transmute(self.0) }
    }
}

impl hash::Hash for RecklessFloat {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.key().hash(state)
    }
}

impl PartialEq for RecklessFloat {
    fn eq(&self, other: &RecklessFloat) -> bool {
        self.key() == other.key()
    }
}

impl Eq for RecklessFloat {}

impl Ord for RecklessFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0)
            .unwrap_or_else(||
                if self.0.is_nan() {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            )
    }
}

impl From<RecklessFloat> for f64 {
    fn from(wrapped_value: RecklessFloat) -> Self {
        wrapped_value.0
    }
}

impl From<&RecklessFloat> for f64 {
    fn from(wrapped_value: &RecklessFloat) -> Self {
        wrapped_value.0
    }
}

impl From<f64> for RecklessFloat {
    fn from(value: f64) -> Self {
        RecklessFloat(value)
    }
}

impl From<&f64> for RecklessFloat {
    fn from(value: &f64) -> Self {
        RecklessFloat(*value)
    }
}