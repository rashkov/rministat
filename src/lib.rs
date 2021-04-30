use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::os::linux::fs::MetadataExt;
use std::thread;
mod work;

pub fn read_to_str_sorted(filepath: &str) -> std::vec::Vec<i32> {
    let mut sorted = vec![];
    for num in fs::read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .lines()
    {
        match num.parse::<i32>() {
            Ok(n) => sorted.push(n),
            Err(_e) => continue,
        }
    }
    sorted.sort();
    sorted
}

pub fn buf_read_sorted(filepath: &str) -> std::vec::Vec<i32> {
    let f = File::open(filepath).unwrap();
    let reader = BufReader::new(f);
    let mut sorted = vec![];
    for num in reader.lines() {
        match num.unwrap().parse::<i32>() {
            Ok(n) => sorted.push(n),
            Err(_e) => continue,
        }
    }
    sorted.sort();
    sorted
}

pub fn mt_sorted(filepath: &str) -> std::vec::Vec<i32> {
    let filesize = fs::metadata(&filepath).unwrap().st_size();
    let nthreads = num_cpus::get();
    let mut threads = std::vec::Vec::new();

    let work = work::align_intervals_to_delim(
        work::intervals(nthreads, filesize as usize),
        &mut File::open(&filepath).unwrap(),
        filesize as usize,
        b'\n',
    );
    for w in work {
        let filepath = String::from(filepath);
        let thread_handle = thread::spawn(move || {
            let mut f = File::open(&filepath).unwrap();
            let (start, end) = w;
            f.seek(SeekFrom::Start(start as u64)).unwrap();
            let len = end - start + 1;
            let mut buf: std::vec::Vec<u8> = vec![0; len];
            f.read(&mut buf[..]).unwrap();

            let mut sorted = vec![];
            for num in buf.lines() {
                match num.unwrap().parse::<i32>() {
                    Ok(n) => sorted.push(n),
                    Err(_e) => continue,
                }
            }
            sorted.sort();
            sorted
        });
        threads.push(thread_handle);
    }
    let mut sorted = vec![];
    for thr in threads {
        let mut thread_sorted = thr.join().unwrap();
        sorted.append(&mut thread_sorted);
    }
    sorted.sort();
    sorted
}

pub fn read_to_str_sum(filepath: &str) -> i128 {
    let mut sum: i128 = 0;
    for num in fs::read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .lines()
    {
        match num.parse::<i32>() {
            Ok(n) => sum += n as i128,
            Err(_e) => continue,
        }
    }
    sum
}

pub fn buf_read_sum(filepath: &str) -> i128 {
    let f = File::open(filepath).unwrap();
    let reader = BufReader::new(f);
    let mut sum: i128 = 0;
    for num in reader.lines() {
        match num.unwrap().parse::<i32>() {
            Ok(n) => sum += n as i128,
            Err(_e) => continue,
        }
    }
    sum
}

pub fn mt_sum(filepath: &str) -> i128 {
    let filesize = fs::metadata(&filepath).unwrap().st_size();
    let nthreads = num_cpus::get();
    let mut threads = std::vec::Vec::new();

    let work = work::align_intervals_to_delim(
        work::intervals(nthreads, filesize as usize),
        &mut File::open(&filepath).unwrap(),
        filesize as usize,
        b'\n',
    );
    for w in work {
        let filepath = String::from(filepath);
        let thread_handle = thread::spawn(move || {
            let mut f = File::open(&filepath).unwrap();
            let (start, end) = w;
            f.seek(SeekFrom::Start(start as u64)).unwrap();
            let len = end - start + 1;
            let mut buf: std::vec::Vec<u8> = vec![0; len];
            f.read(&mut buf[..]).unwrap();

            let mut sum: i128 = 0;
            for num in buf.lines() {
                match num.unwrap().parse::<i32>() {
                    Ok(n) => sum += n as i128,
                    Err(_e) => continue,
                }
            }
            sum
        });
        threads.push(thread_handle);
    }
    let mut sum: i128 = 0;
    for thr in threads {
        let thread_sum = thr.join().unwrap();
        sum += thread_sum;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_to_str_works() {
        let path = ["./test/data/", "1024", ".txt"].join("");
        let sum = read_to_str_sum(&path);
        assert_eq!(sum, 4936211233);
    }

    #[test]
    fn buf_read_works() {
        let path = ["./test/data/", "1024", ".txt"].join("");
        let sum = buf_read_sum(&path);
        assert_eq!(sum, 4936211233);
    }

    #[test]
    fn mt_sum_works() {
        let path = ["./test/data/", "1024", ".txt"].join("");
        let sum = mt_sum(&path);
        assert_eq!(sum, 4936211233);
    }

    #[test]
    fn read_to_str_sorted_works() {
        let path = ["./test/data/", "1024", ".txt"].join("");
        let sorted = read_to_str_sorted(&path);
        let mut previous = &sorted[0];
        for n in &sorted {
            assert!(n >= previous);
            previous = n;
        }
    }

    #[test]
    fn buf_read_sorted_works() {
        let path = ["./test/data/", "1024", ".txt"].join("");
        let sorted = buf_read_sorted(&path);
        let mut previous = &sorted[0];
        for n in &sorted {
            assert!(n >= previous);
            previous = n;
        }
    }

    #[test]
    fn mt_sorted_works() {
        let path = ["./test/data/", "1024", ".txt"].join("");
        let sorted = mt_sorted(&path);
        let mut previous = &sorted[0];
        for n in &sorted {
            assert!(n >= previous);
            previous = n;
        }
    }
}
