// Split a range of interval [0,r) into n equal buckets
// slop goes to the last bucket
// interval gives the range of a given bucket
#[allow(dead_code)]
pub fn interval(bucket: usize, buckets: usize, r: usize) -> (usize, usize) {
    // map bucket 0 to start=0
    // map bucket 1 to start=3
    // map bucket 2 to start=6
    // pattern above is bucket_size * bucket

    // map bucket 0 to end=2
    // map bucket 1 to end=5
    // map bucket 2 to end=9
    // pattern above is start + bucket_size - 1

    // calculate bucket_size and slop
    let bucket_size = r / buckets;
    let slop = r - bucket_size * buckets;

    let start = bucket_size * bucket;
    let end = start + bucket_size - 1;

    // allocate slop to the last bucket
    if bucket == buckets - 1 {
        (start, end + slop)
    } else {
        (start, end)
    }
}

#[allow(dead_code)]
pub fn intervals(buckets: usize, r: usize) -> Vec<(usize, usize)> {
    (0..buckets)
        .map(|b| interval(b, buckets, r))
        .collect::<Vec<(usize, usize)>>()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_interval() {
        // split [0,10) into 3 equal buckets, allocating slop to the last bucket
        let r = 10;
        let buckets = 3;
        assert_eq!(interval(0, buckets, r), (0, 2));
        assert_eq!(interval(1, buckets, r), (3, 5));
        assert_eq!(interval(2, buckets, r), (6, 9));
    }

    #[test]
    fn test_intervals() {
        // split [0,10) into 3 equal buckets, allocating slop to the last bucket
        assert_eq!(vec![(0, 2), (3, 5), (6, 9)], intervals(3, 10));
    }
}
