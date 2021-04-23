use std::convert::TryFrom;
use std::env;
use std::fs;

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

    // Compute and print avg
    let mut total = 0;
    let count: i128 =
        i128::try_from(nums.len()).expect("Unable to convert count of lines (usize) to i128");
    for num in nums {
        let (res, overflow) = i128::overflowing_add(total, i128::try_from(num).unwrap());
        if overflow {
            panic!("Overflow while computing average");
        }
        total = res;
    }
    println!("avg {}", total / i128::try_from(count).unwrap());
}
