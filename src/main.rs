use num_cpus;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::thread;
use std::sync::Arc;

fn main() {
    // Read in file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./ministat filename.txt");
        std::process::exit(1);
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Parse file to vec
    let mut nums: Vec<i32> = std::vec::Vec::new();
    for num in contents.lines() {
        match num.parse::<i32>() {
            Ok(n) => nums.push(n),
            Err(_e) => continue,
        }
    }

    let count: i128 =
        i128::try_from(nums.len()).expect("Unable to convert count of lines (usize) to i128");
    // // Compute and print avg
    // let mut total = 0;
    // for &num in &nums {
    //     let (res, overflow) = i128::overflowing_add(total, i128::try_from(num).unwrap());
    //     if overflow {
    //         panic!("Overflow while computing average");
    //     }
    //     total = res;
    // }
    // println!("avg {}", total / i128::try_from(count).unwrap());

    // compute number of threads
    let nthreads = i128::try_from(num_cpus::get()).unwrap();
    let work_per_thread = count / nthreads;
    let slop = count - nthreads * work_per_thread;
    println!("Total lines: {}", count);
    println!(
        "Creating {} threads, each processing {} lines, with slop of {} going to the last thread",
        nthreads, work_per_thread, slop
    );

    let mut threads = std::vec::Vec::new();
    let nums_arc = Arc::new(nums);
    for t in 0..nthreads {
        let nums = nums_arc.clone();
        let ti32 = i32::try_from(t).unwrap();
        let nthreadsi32 = i32::try_from(nthreads).unwrap();
        let counti32 = i32::try_from(count).unwrap();
        let (start, end) = interval(ti32, nthreadsi32, counti32);
        let start = usize::try_from(start).unwrap();
        let end = usize::try_from(end).unwrap();
        let thread_handle = thread::spawn(move || {
            let work = &nums[start..end];
            let mut sum: i128 = 0;
            for num in work {
                sum += i128::try_from(*num).unwrap();
            }
            sum
        });
        threads.push(thread_handle);
    }

    let mut sum: i128 = 0;
    for thr in threads {
        sum += thr.join().unwrap();
    }
    println!("Avg: {}", sum / count);
}

// Split a range of interval [0,r) into n equal buckets
// slop goes to the last bucket
// interval gives the range of a given bucket
#[allow(dead_code)]
fn interval(bucket: i32, buckets: i32, r: i32) -> (i32, i32) {
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
}
