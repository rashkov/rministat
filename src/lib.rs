use std::convert::TryFrom;
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

/* Notes:
 *   TODO: benchmark std::vec::Vec::select_nth_unstable instead of sorting
 *   TODO: benchmark sort_unstable() instead of sort()
 *   TODO: re-evaluate correctness of casts such as "i128 as f64"
 */

const NSTUDENT: usize = 100;
const NCONF: usize = 6;
const STUDENTPCT: [f64; NCONF] = [80., 90., 95., 98., 99., 99.5];
const STUDENT: [[f64; NCONF]; NSTUDENT + 1] = [
    /* inf */ [1.282, 1.645, 1.960, 2.326, 2.576, 3.090],
    /* 1. */ [3.078, 6.314, 12.706, 31.821, 63.657, 318.313],
    /* 2. */ [1.886, 2.920, 4.303, 6.965, 9.925, 22.327],
    /* 3. */ [1.638, 2.353, 3.182, 4.541, 5.841, 10.215],
    /* 4. */ [1.533, 2.132, 2.776, 3.747, 4.604, 7.173],
    /* 5. */ [1.476, 2.015, 2.571, 3.365, 4.032, 5.893],
    /* 6. */ [1.440, 1.943, 2.447, 3.143, 3.707, 5.208],
    /* 7. */ [1.415, 1.895, 2.365, 2.998, 3.499, 4.782],
    /* 8. */ [1.397, 1.860, 2.306, 2.896, 3.355, 4.499],
    /* 9. */ [1.383, 1.833, 2.262, 2.821, 3.250, 4.296],
    /* 10. */ [1.372, 1.812, 2.228, 2.764, 3.169, 4.143],
    /* 11. */ [1.363, 1.796, 2.201, 2.718, 3.106, 4.024],
    /* 12. */ [1.356, 1.782, 2.179, 2.681, 3.055, 3.929],
    /* 13. */ [1.350, 1.771, 2.160, 2.650, 3.012, 3.852],
    /* 14. */ [1.345, 1.761, 2.145, 2.624, 2.977, 3.787],
    /* 15. */ [1.341, 1.753, 2.131, 2.602, 2.947, 3.733],
    /* 16. */ [1.337, 1.746, 2.120, 2.583, 2.921, 3.686],
    /* 17. */ [1.333, 1.740, 2.110, 2.567, 2.898, 3.646],
    /* 18. */ [1.330, 1.734, 2.101, 2.552, 2.878, 3.610],
    /* 19. */ [1.328, 1.729, 2.093, 2.539, 2.861, 3.579],
    /* 20. */ [1.325, 1.725, 2.086, 2.528, 2.845, 3.552],
    /* 21. */ [1.323, 1.721, 2.080, 2.518, 2.831, 3.527],
    /* 22. */ [1.321, 1.717, 2.074, 2.508, 2.819, 3.505],
    /* 23. */ [1.319, 1.714, 2.069, 2.500, 2.807, 3.485],
    /* 24. */ [1.318, 1.711, 2.064, 2.492, 2.797, 3.467],
    /* 25. */ [1.316, 1.708, 2.060, 2.485, 2.787, 3.450],
    /* 26. */ [1.315, 1.706, 2.056, 2.479, 2.779, 3.435],
    /* 27. */ [1.314, 1.703, 2.052, 2.473, 2.771, 3.421],
    /* 28. */ [1.313, 1.701, 2.048, 2.467, 2.763, 3.408],
    /* 29. */ [1.311, 1.699, 2.045, 2.462, 2.756, 3.396],
    /* 30. */ [1.310, 1.697, 2.042, 2.457, 2.750, 3.385],
    /* 31. */ [1.309, 1.696, 2.040, 2.453, 2.744, 3.375],
    /* 32. */ [1.309, 1.694, 2.037, 2.449, 2.738, 3.365],
    /* 33. */ [1.308, 1.692, 2.035, 2.445, 2.733, 3.356],
    /* 34. */ [1.307, 1.691, 2.032, 2.441, 2.728, 3.348],
    /* 35. */ [1.306, 1.690, 2.030, 2.438, 2.724, 3.340],
    /* 36. */ [1.306, 1.688, 2.028, 2.434, 2.719, 3.333],
    /* 37. */ [1.305, 1.687, 2.026, 2.431, 2.715, 3.326],
    /* 38. */ [1.304, 1.686, 2.024, 2.429, 2.712, 3.319],
    /* 39. */ [1.304, 1.685, 2.023, 2.426, 2.708, 3.313],
    /* 40. */ [1.303, 1.684, 2.021, 2.423, 2.704, 3.307],
    /* 41. */ [1.303, 1.683, 2.020, 2.421, 2.701, 3.301],
    /* 42. */ [1.302, 1.682, 2.018, 2.418, 2.698, 3.296],
    /* 43. */ [1.302, 1.681, 2.017, 2.416, 2.695, 3.291],
    /* 44. */ [1.301, 1.680, 2.015, 2.414, 2.692, 3.286],
    /* 45. */ [1.301, 1.679, 2.014, 2.412, 2.690, 3.281],
    /* 46. */ [1.300, 1.679, 2.013, 2.410, 2.687, 3.277],
    /* 47. */ [1.300, 1.678, 2.012, 2.408, 2.685, 3.273],
    /* 48. */ [1.299, 1.677, 2.011, 2.407, 2.682, 3.269],
    /* 49. */ [1.299, 1.677, 2.010, 2.405, 2.680, 3.265],
    /* 50. */ [1.299, 1.676, 2.009, 2.403, 2.678, 3.261],
    /* 51. */ [1.298, 1.675, 2.008, 2.402, 2.676, 3.258],
    /* 52. */ [1.298, 1.675, 2.007, 2.400, 2.674, 3.255],
    /* 53. */ [1.298, 1.674, 2.006, 2.399, 2.672, 3.251],
    /* 54. */ [1.297, 1.674, 2.005, 2.397, 2.670, 3.248],
    /* 55. */ [1.297, 1.673, 2.004, 2.396, 2.668, 3.245],
    /* 56. */ [1.297, 1.673, 2.003, 2.395, 2.667, 3.242],
    /* 57. */ [1.297, 1.672, 2.002, 2.394, 2.665, 3.239],
    /* 58. */ [1.296, 1.672, 2.002, 2.392, 2.663, 3.237],
    /* 59. */ [1.296, 1.671, 2.001, 2.391, 2.662, 3.234],
    /* 60. */ [1.296, 1.671, 2.000, 2.390, 2.660, 3.232],
    /* 61. */ [1.296, 1.670, 2.000, 2.389, 2.659, 3.229],
    /* 62. */ [1.295, 1.670, 1.999, 2.388, 2.657, 3.227],
    /* 63. */ [1.295, 1.669, 1.998, 2.387, 2.656, 3.225],
    /* 64. */ [1.295, 1.669, 1.998, 2.386, 2.655, 3.223],
    /* 65. */ [1.295, 1.669, 1.997, 2.385, 2.654, 3.220],
    /* 66. */ [1.295, 1.668, 1.997, 2.384, 2.652, 3.218],
    /* 67. */ [1.294, 1.668, 1.996, 2.383, 2.651, 3.216],
    /* 68. */ [1.294, 1.668, 1.995, 2.382, 2.650, 3.214],
    /* 69. */ [1.294, 1.667, 1.995, 2.382, 2.649, 3.213],
    /* 70. */ [1.294, 1.667, 1.994, 2.381, 2.648, 3.211],
    /* 71. */ [1.294, 1.667, 1.994, 2.380, 2.647, 3.209],
    /* 72. */ [1.293, 1.666, 1.993, 2.379, 2.646, 3.207],
    /* 73. */ [1.293, 1.666, 1.993, 2.379, 2.645, 3.206],
    /* 74. */ [1.293, 1.666, 1.993, 2.378, 2.644, 3.204],
    /* 75. */ [1.293, 1.665, 1.992, 2.377, 2.643, 3.202],
    /* 76. */ [1.293, 1.665, 1.992, 2.376, 2.642, 3.201],
    /* 77. */ [1.293, 1.665, 1.991, 2.376, 2.641, 3.199],
    /* 78. */ [1.292, 1.665, 1.991, 2.375, 2.640, 3.198],
    /* 79. */ [1.292, 1.664, 1.990, 2.374, 2.640, 3.197],
    /* 80. */ [1.292, 1.664, 1.990, 2.374, 2.639, 3.195],
    /* 81. */ [1.292, 1.664, 1.990, 2.373, 2.638, 3.194],
    /* 82. */ [1.292, 1.664, 1.989, 2.373, 2.637, 3.193],
    /* 83. */ [1.292, 1.663, 1.989, 2.372, 2.636, 3.191],
    /* 84. */ [1.292, 1.663, 1.989, 2.372, 2.636, 3.190],
    /* 85. */ [1.292, 1.663, 1.988, 2.371, 2.635, 3.189],
    /* 86. */ [1.291, 1.663, 1.988, 2.370, 2.634, 3.188],
    /* 87. */ [1.291, 1.663, 1.988, 2.370, 2.634, 3.187],
    /* 88. */ [1.291, 1.662, 1.987, 2.369, 2.633, 3.185],
    /* 89. */ [1.291, 1.662, 1.987, 2.369, 2.632, 3.184],
    /* 90. */ [1.291, 1.662, 1.987, 2.368, 2.632, 3.183],
    /* 91. */ [1.291, 1.662, 1.986, 2.368, 2.631, 3.182],
    /* 92. */ [1.291, 1.662, 1.986, 2.368, 2.630, 3.181],
    /* 93. */ [1.291, 1.661, 1.986, 2.367, 2.630, 3.180],
    /* 94. */ [1.291, 1.661, 1.986, 2.367, 2.629, 3.179],
    /* 95. */ [1.291, 1.661, 1.985, 2.366, 2.629, 3.178],
    /* 96. */ [1.290, 1.661, 1.985, 2.366, 2.628, 3.177],
    /* 97. */ [1.290, 1.661, 1.985, 2.365, 2.627, 3.176],
    /* 98. */ [1.290, 1.661, 1.984, 2.365, 2.627, 3.175],
    /* 99. */ [1.290, 1.660, 1.984, 2.365, 2.626, 3.175],
    /* 100. */ [1.290, 1.660, 1.984, 2.364, 2.626, 3.174],
];
const MAX_DS: usize = 8;
const symbol: [char; MAX_DS] = [' ', 'x', '+', '*', '%', '#', '@', 'O'];

pub struct DataSet<'a> {
    name: &'a str,
    pub points: std::vec::Vec<i64>,
    sy: i128,
    syy: i128,
}

impl DataSet<'_> {
    pub fn new(name: &str) -> DataSet {
        DataSet {
            name,
            points: vec![],
            sy: 0,
            syy: 0
        }
    }
    pub fn add_points(&mut self, points: &[i64]){
        for &point in points {
            self.points.push(point);
            self.sy += point as i128;
            self.syy += (point * point) as i128;
        }
    }
    fn add_point(&mut self, point: i64) {
        self.points.push(point);
        self.sy += point as i128;
        self.syy += (point * point) as i128;
    }
    fn sort(&mut self) {
        self.points.sort();
    }
    fn len(&self) -> i128 {
        self.points.len() as i128
    }
    pub fn min(&self) -> Option<&i64> {
        self.points.first()
    }
    pub fn max(&self) -> Option<&i64> {
        self.points.last()
    }
    fn median(&self) -> Option<&i64> {
        let l = self.len() as usize;
        self.points.get(l / 2)
    }
    pub fn avg(&self) -> i128 {
        self.sy / (self.len() as i128)
    }
    fn var(&self) -> i128 {
        (self.syy - self.sy * self.sy / self.len()) / (self.len() - 1)
    }
    pub fn stddev(&self) -> f64 {
        match i64::try_from(self.var()) {
            Ok(v) => (v as f64).sqrt(),
            Err(e) => {
                eprintln!("Unable to compute variance & stddev: {}", e);
                0.0
            }
        }
    }
    pub fn relative(&self, other: &DataSet, confidx: usize) {
        let i = self.len() + other.len() - 2;
        let mut spool: f64;
        let s: f64;
        let d: f64;
        let e: f64;
        let t: f64;
        if i > NSTUDENT as i128 {
            t = STUDENT[0][confidx];
        } else {
            t = STUDENT[i as usize][confidx];
        }
        // XXX: how do I ensure correctness when using "as f64" ?
        spool = ((self.len() - 1) * (self.var() as i128)
            + (other.len() - 1) * (other.var() as i128)) as f64;
        spool /= (self.len() + other.len() - 2) as f64;
        spool = spool.sqrt();
        s = spool * (1. / (self.len() as f64) + 1. / (other.len() as f64)).sqrt();
        d = (self.avg() - other.avg()) as f64;
        e = t * s;

        if d.abs() > e {
            println!("Difference at {:.1} confidence", STUDENTPCT[confidx]);
            println!("	{} +/- {}", d, e);
            println!(
                "	{}% +/- {}%",
                d * 100. / (self.avg() as f64),
                e * 100. / (other.avg() as f64)
            );
            println!("	(Student's t, pooled s = {})", spool);
        } else {
            println!(
                "No difference proven at {:.1}% confidence\n",
                STUDENTPCT[confidx]
            );
        }
    }
    pub fn vitals(&self) {
        let flag = 1;
        println!("    N           Min           Max        Median           Avg        Stddev");
        println!(
            // "%c %3d %13.8g %13.8g %13.8g %13.8g %13.8g",
            "{} {:3} {:13.8} {:13.8} {:13.8} {:13.8} {:13.8}",
            symbol[flag],
            self.len(),
            self.min().unwrap_or(&0),
            self.max().unwrap_or(&0),
            self.median().unwrap_or(&0),
            self.avg(),
            self.stddev()
        );
        println!();
    }
}

pub fn readset_st(filepath: &str) -> DataSet {
    let mut ds = DataSet {
        name: filepath,
        points: vec![],
        sy: 0,
        syy: 0,
    };
    for num in fs::read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .lines()
    {
        match num.parse::<i64>() {
            Ok(n) => ds.add_point(n),
            Err(_e) => continue,
        }
    }
    ds.sort();
    ds
}

pub fn readset_mt(filepath: &str) -> DataSet {
    let mut ds = DataSet {
        name: filepath,
        points: vec![],
        sy: 0,
        syy: 0,
    };
    for num in fs::read_to_string(filepath)
        .expect("Something went wrong reading the file")
        .lines()
    {
        match num.parse::<i64>() {
            Ok(n) => ds.add_point(n),
            Err(_e) => continue,
        }
    }
    ds.sort();
    ds
}

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

    #[test]
    fn add_points_works(){
        let mut ds = DataSet::new("foo");
        ds.add_points(&[1,2,3]);
        assert!(ds.len() == 3);
        assert!(ds.sy == 6);
        assert!(ds.syy == 14);
    }
}
