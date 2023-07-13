use std::collections::HashMap;

use ordered_float::OrderedFloat;

pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn median(data: &mut [f64]) -> f64 {
    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let len = data.len();
    if len % 2 == 0 {
        let mid1 = data[len / 2 - 1];
        let mid2 = data[len / 2];
        (mid1 + mid2) / 2.0
    } else {
        data[len / 2]
    }
}

pub fn mode(data: &[f64]) -> Option<f64> {
    let mut frequency_map = HashMap::<OrderedFloat<f64>, i32>::new();
    for &value in data {
        let count = frequency_map.entry(OrderedFloat(value)).or_insert(0);
        *count += 1;
    }
    frequency_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value.0)
}
