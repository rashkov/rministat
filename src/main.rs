use num_cpus;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::sync::Arc;
use std::thread;
mod work;

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

    let count = nums.len();
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
    let nthreads = num_cpus::get();
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
        let (start, end) = work::interval(t, nthreads, count);
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
    println!("Avg: {}", sum / count as i128);
}
