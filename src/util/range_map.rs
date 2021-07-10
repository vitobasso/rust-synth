use crate::util::reckless_float::RecklessFloat;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Maps ranges of f64 to values.
/// f64 keys in the map are cycled around when queried with range(from, to)
///     e.g. With state [0.0, 0.5, end:1.0], range(2.0, 3.0) returns [0.0, 0.5] from a 2nd cycle.
/// Same key can have multiple values.
/// Keys can't be negative
#[derive(Clone, PartialEq, Debug)]
pub struct CyclicRangeMap<T> {
    tree_map: BTreeMap<RecklessFloat, Vec<T>>,
    end: f64
}

impl <T> CyclicRangeMap<T> {
    pub fn new(source: Vec<(f64, T)>, end: f64) -> Self {
        let map: BTreeMap<RecklessFloat, Vec<T>> = source.into_iter()
            .map(|(k, v)| (RecklessFloat::from(k), v))
            .fold(HashMap::<RecklessFloat, Vec<T>>::default(), |mut map, (k, v)| {
                map.entry(k).or_insert_with(Vec::default).push(v);
                map
            }).into_iter().collect();
        CyclicRangeMap { tree_map: map, end }
    }

    pub fn full_cycle(&self) -> Vec<&T> {
        self.range(0., self.end)
    }

    pub fn end(&self) -> f64 {
        self.end
    }

    pub fn range(&self, from: f64, to: f64) -> Vec<&T> {
        let full_cycles = ((to - from) / self.end).floor() as usize;
        let cycled_from = from % self.end;
        let cycled_to = to % self.end;

        if to < from {
            vec![]
        } else if full_cycles > 0 {
            let middle = self.tree_map.iter()
                .flat_map(|(_, v)| v).cycle().take(self.tree_map.len() * full_cycles);
            let end = self.tree_map
                .range(..RecklessFloat(cycled_to))
                .flat_map(|(_, v)| v);
            let tail = middle.chain(end);
            if cycled_from > 0. {
                let begin = self.tree_map
                    .range(RecklessFloat(cycled_from)..)
                    .flat_map(|(_, v)| v);
                begin.chain(tail).collect()
            } else {
                tail.collect()
            }
        } else if cycled_to < cycled_from {
            let begin = self.tree_map
                .range(RecklessFloat(cycled_from)..)
                .flat_map(|(_, v)| v);
            let end = self.tree_map
                .range(..RecklessFloat(cycled_to))
                .flat_map(|(_, v)| v);
            begin.chain(end).collect()
        } else {
            self.tree_map
                .range(RecklessFloat(cycled_from)..RecklessFloat(cycled_to))
                .flat_map(|(_, v)| v).collect()
        }

    }

    pub fn into_tuples(self) -> Vec<(f64, T)> {
        self.tree_map.into_iter()
            .flat_map(|(k,list)| list.into_iter().map(move |item| (f64::from(k), item)))
            .collect()
    }
}

impl <T> Default for CyclicRangeMap<T> {
    fn default() -> Self {
        CyclicRangeMap {
            tree_map: BTreeMap::default(),
            end: 0.
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn seq_basic() -> CyclicRangeMap<String> {
        CyclicRangeMap::new(vec!(
            (0.,   "A".to_string()),
            (0.5,  "B".to_string()),
            (0.75, "C".to_string()),
        ), 1.)
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
    fn crossing_cycle() {
        assert_eq!(seq_basic().range(0.9, 1.1), vec!("A"))
    }

    #[test]
    fn from_gt_to() {
        assert_eq!(seq_basic().range(0.9, 0.1), Vec::<&String>::default())
    }

    #[test]
    fn second_cycle() {
        assert_eq!(seq_basic().range(1., 2.), vec!("A", "B", "C"))
    }

    #[test]
    fn multiple_cycles() {
        assert_eq!(seq_basic().range(-1., 2.), vec!("A", "B", "C", "A", "B", "C", "A", "B", "C"))
    }


    fn seq_overlapping() -> CyclicRangeMap<String> {
        CyclicRangeMap::new(vec!(
            (0., "A".to_string()),
            (0., "B".to_string()),
        ), 1.)
    }

    #[test]
    fn overlapping() {
        assert_eq!(seq_overlapping().range(-1., 1.), vec!("A", "B"))
    }

}