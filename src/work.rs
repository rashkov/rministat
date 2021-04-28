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

use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;
#[allow(dead_code)]
pub fn align_intervals_to_delim<T: std::io::Read + std::io::Seek>(
    intervals: Vec<(usize, usize)>,
    data: &mut T,
    length: usize,
    delim: u8,
) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = vec![];
    let mut reader = BufReader::new(data);
    for &(mut new_start, mut new_end) in intervals.iter() {
        match ret.last() {
            Some(&(last_start, last_end)) => {
                // check whether the prior interval overlaps the current
                // then shift its endpoint to the next newline

                // We've already accounted for the entire length of data
                if last_end == length - 1 {
                    continue;
                }

                // Fully overlapped, skip this interval
                if new_end <= last_start {
                    continue;
                }

                //Partially overlapped:
                if new_start <= last_end {
                    // set the new_start to just past last_end
                    new_start = last_end + 1;
                    // make sure new_end is not behind new_start
                    if new_end < new_start {
                        new_end = new_start;
                    }
                }
            }
            None => (),
        };

        // shift the end to the next newline
        let mut buf = vec![];
        reader.seek(SeekFrom::Start(new_end as u64)).unwrap();
        let n = reader.read_until(delim, &mut buf).unwrap();
        new_end = new_end + n - 1;
        ret.push((new_start, new_end))
    }
    ret
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

    #[test]
    fn test_line_intervals() {
        // several test scenarios:
        // 1. all contents on one line
        //    a. there is no separator
        //    b. there is one separator at end-of-file
        // 2. one interval fully overlaps another interval, and partially overlaps another interval
        //    a. one interval fully overlaps another interval
        //    b. one interval partially overlaps another interval
        //    c. one inteval fuly overlaps one interval, AND partially overlaps another interval
        // 3. intervals that are already perfect should not be modified

        use std::io::Cursor;

        // 1a: all contents on one line, and there is no separator
        let str_1a = b"aab";
        let cursor_1a = Cursor::new(str_1a);
        let mut reader_1a = BufReader::new(cursor_1a);
        let unaligned_1a = intervals(2, str_1a.len());
        assert_eq!(unaligned_1a, [(0, 0), (1, 2)]);
        let aligned_1a =
            align_intervals_to_delim(unaligned_1a, &mut reader_1a, str_1a.len(), b'\n');
        assert_eq!(aligned_1a, [(0, 2)]);

        // 1b: all contents on one line, separator at end-of-file
        let str_1b = b"aab\n";
        let cursor_1b = Cursor::new(str_1b);
        let mut reader_1b = BufReader::new(cursor_1b);
        let unaligned_1b = intervals(2, str_1b.len());
        assert_eq!(unaligned_1b, [(0, 1), (2, 3)]);
        let aligned_1b =
            align_intervals_to_delim(unaligned_1b, &mut reader_1b, str_1b.len(), b'\n');
        assert_eq!(aligned_1b, [(0, 3)]);

        // 2a: one interval fully overlaps another interval
        // we use three buckets for this one
        // original buckets are ("aab", "ccc", "\ndd")
        // aligned buckets should be ("aabccc\n", "dd")
        let str_2a = b"aabccc\ndd"; // length == 9
        let nbuckets_2a = 3;
        let cursor_2a = Cursor::new(str_2a);
        let mut reader_2a = BufReader::new(cursor_2a);
        let unaligned_2a = intervals(nbuckets_2a, str_2a.len());
        assert_eq!(unaligned_2a, [(0, 2), (3, 5), (6, 8)]);
        let aligned_2a =
            align_intervals_to_delim(unaligned_2a, &mut reader_2a, str_2a.len(), b'\n');
        assert_eq!(aligned_2a, [(0, 6), (7, 8)]);

        // 2b: one interval partially overlaps another interval
        // original buckets are ("aab", "c\nc", "\ndd")
        // aligned buckets should be ("aabc\n", "c\n", "dd")
        let str_2b = b"aabc\nc\ndd"; // length == 9
        let nbuckets_2b = 3;
        let cursor_2b = Cursor::new(str_2b);
        let mut reader_2b = BufReader::new(cursor_2b);
        let unaligned_2b = intervals(nbuckets_2b, str_2b.len());
        assert_eq!(unaligned_2b, [(0, 2), (3, 5), (6, 8)]);
        let aligned_2b =
            align_intervals_to_delim(unaligned_2b, &mut reader_2b, str_2b.len(), b'\n');
        assert_eq!(aligned_2b, [(0, 4), (5, 6), (7, 8)]);

        // 2 c. one interval fully overlaps one interval, AND partially overlaps another interval
        // original buckets are ("aab", "ccc", "d\nd")
        // aligned buckets should be ("aabcccd\n", "d")
        let str_2c = b"aabcccd\nd"; // length == 9
        let nbuckets_2c = 3;
        let cursor_2c = Cursor::new(str_2c);
        let mut reader_2c = BufReader::new(cursor_2c);
        let unaligned_2c = intervals(nbuckets_2c, str_2c.len());
        assert_eq!(unaligned_2c, [(0, 2), (3, 5), (6, 8)]);
        let aligned_2c =
            align_intervals_to_delim(unaligned_2c, &mut reader_2c, str_2c.len(), b'\n');
        assert_eq!(aligned_2c, [(0, 7), (8, 8)]);

        // 3. intervals are already perfect, and should not shift
        // original buckets are ("\n", "ab")
        // aligned buckets should be ("\n", "ab")
        let str_3a = b"\nab"; // length == 3
        let nbuckets_3a = 2;
        let cursor_3a = Cursor::new(str_3a);
        let mut reader_3a = BufReader::new(cursor_3a);
        let unaligned_3a = intervals(nbuckets_3a, str_3a.len());
        assert_eq!(unaligned_3a, [(0, 0), (1, 2)]);
        let aligned_3a =
            align_intervals_to_delim(unaligned_3a, &mut reader_3a, str_3a.len(), b'\n');
        assert_eq!(aligned_3a, [(0, 0), (1, 2)]);
    }
}
