use crate::util::reckless_float::RecklessFloat;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Maps ranges of f64 to values.
/// Keys are cyclic.
/// Same key can have multiple values.
#[derive(Clone, PartialEq, Debug)]
pub struct RangeMap<T> {
    tree_map: BTreeMap<RecklessFloat, Vec<T>>,
}

impl <T> RangeMap<T> {
    pub fn new(source: Vec<(f64, T)>) -> Self {
        let map: BTreeMap<RecklessFloat, Vec<T>> = source.into_iter()
            .map(|(k, v)| (RecklessFloat::from(k), v))
            .fold(HashMap::<RecklessFloat, Vec<T>>::default(), |mut map, (k, v)| {
                map.entry(k).or_insert_with(Vec::default).push(v);
                map
            }).into_iter().collect();
        RangeMap { tree_map: map }
    }

    pub fn range(&self, from: f64, to: f64) -> Vec<&T> {
        if from <= to {
            self.tree_map
                .range(RecklessFloat(from)..RecklessFloat(to))
                .flat_map(|(_, v)| v).collect()
        } else {
            let first_half = self.tree_map
                .range(RecklessFloat(from)..)
                .flat_map(|(_, v)| v);
            let second_half = self.tree_map
                .range(..RecklessFloat(to))
                .flat_map(|(_, v)| v);
            first_half.chain(second_half).collect()
        }
    }

    pub fn into_tuples(self) -> Vec<(f64, T)> {
        self.tree_map.into_iter()
            .flat_map(|(k,list)| list.into_iter().map(move |item| (f64::from(k), item)))
            .collect()
    }
}

impl <T> Default for RangeMap<T> {
    fn default() -> Self {
        RangeMap {
            tree_map: BTreeMap::default()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn seq_basic() -> RangeMap<String> {
        RangeMap::new(vec!(
            (0.,   "A".to_string()),
            (0.5,  "B".to_string()),
            (0.75, "C".to_string()),
        ))
    }

    #[test]
    fn whole() {
        assert_eq!(seq_basic().range(0., 1.), vec!("A", "B", "C"))
    }

    #[test]
    fn beginning() {
        assert_eq!(seq_basic().range(0., 0.5), vec!("A"))
    }

    #[test]
    fn end() {
        assert_eq!(seq_basic().range(0.4, 1.), vec!("B", "C"))
    }

    #[test]
    fn middle() {
        assert_eq!(seq_basic().range(0.5, 0.6), vec!("B"))
    }

    #[test]
    fn wrapping() {
        assert_eq!(seq_basic().range(0.9, 0.1), vec!("A"))
    }

    #[test]
    fn out_of_bounds() {
        assert_eq!(seq_basic().range(1., 2.), Vec::<&String>::default())
    }

    #[test]
    fn extrapolating() {
        assert_eq!(seq_basic().range(-1., 2.), vec!("A", "B", "C"))
    }


    fn seq_overlapping() -> RangeMap<String> {
        RangeMap::new(vec!(
            (0., "A".to_string()),
            (0.,   "B".to_string()),
        ))
    }

    #[test]
    fn overlapping() {
        assert_eq!(seq_overlapping().range(-1., 1.), vec!("A", "B"))
    }

}