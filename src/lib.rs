#![feature(test)]
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

extern crate test;

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
    use paste::paste;
    use test::Bencher;

    #[macro_export]
    macro_rules! genTest {
        ( $( $x:expr),* ) => {
            paste! {
                $(
                    #[bench]
                    fn [<_ $x _bench_buf_read>](b: &mut Bencher) {
                        let a = stringify!($x);
                        let path = ["./test/data/", a, ".txt"].join("");
                        b.iter(|| buf_read_sum(&path));
                    }

                    #[bench]
                    fn [<_ $x _bench_mt_read>](b: &mut Bencher) {
                        let a = stringify!($x);
                        let path = ["./test/data/", a, ".txt"].join("");
                        b.iter(|| mt_sum(&path));
                    }

                    #[bench]
                    fn [<_ $x _bench_read>](b: &mut Bencher) {
                        let a = stringify!($x);
                        let path = ["./test/data/", a, ".txt"].join("");
                        b.iter(|| read_to_str_sum(&path));
                    }
                )*
            }
        };
    }

    genTest![
        128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288,
        1048576, 2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728
    ];

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
}
