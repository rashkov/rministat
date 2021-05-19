pub struct Plot<'a> {
    min: f64,
    max: f64,
    span: f64,
    width: i32,
    x0: f64,
    dx: f64,
    height: i32,
    data: Option<&'a str>,
    bar: Option<String>,
    separate_bars: bool,
    num_datasets: i32,
}

impl Plot<'_> {
    pub fn new<'a>(width: i32, separate: bool, num_datasets: i32) -> Plot<'a> {
        Plot {
            width: width,
            height: 0,
            data: None,
            bar: None,
            separate_bars: separate,
            num_datasets: num_datasets,
            min: f64::MAX,
            max: f64::MIN,
            dx: 0.,
            span: 0.,
            x0: 0.,
        }
    }
    fn adj_plot(&mut self, a: f64) {
        if a < self.min {
            self.min = a;
        }
        if a > self.max {
            self.max = a;
        }
        self.span = self.max - self.min;
        self.dx = self.span / (self.width as f64 - 1.);
        self.x0 = self.min - 0.5 * self.dx;
    }
    pub fn dim_plot(&mut self, ds: &ministat::DataSet) {
        if let Some(&min) = ds.min() {
            self.adj_plot(min as f64);
        }
        if let Some(&max) = ds.max() {
            self.adj_plot(max as f64);
        }
        self.adj_plot((ds.avg() - (ds.stddev() as i128)) as f64);
        self.adj_plot((ds.avg() + (ds.stddev() as i128)) as f64);
    }
    pub fn plot_set(&mut self, ds: ministat::DataSet, val: i32) {
        self.dim_plot(&ds);
        let mut largest_bucket_size = 1;
        let mut last_bucket = -1;
        let mut current_bucket_count = 0;
        let mut current_bucket;

        // iterate through dataset to allocate enough memory for grid data
        for point in ds.points {
            current_bucket = ((point as f64 - self.x0) / self.dx) as i64; // truncate
            if current_bucket == last_bucket {
                current_bucket_count += 1;
                if current_bucket_count > largest_bucket_size {
                    largest_bucket_size = current_bucket_count;
                }
            } else {
                current_bucket_count = 1;
                last_bucket = current_bucket;
            }
        }
        largest_bucket_size += 1;
        if largest_bucket_size > self.height {
            self.height = largest_bucket_size;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test() {
        let a = vec!['a', 'b', 'c'];
        let b: String = a.into_iter().collect();
        assert_eq!(b, "abc");
    }

    // #[test]
    // fn test2() {
    //     let mut vikings = HashMap::new();
    //     vikings.insert(42, "fooobar");
    //     let a = vikings.get(&42);
    // }

    // DS1: Sample dataset {2, 3, 10}
    // Minimum  min =  2
    // Maximum  max =  10
    // Range  R =  8
    // Count  n =  3
    // Sum  sum =  15
    // Mean  x¯¯¯ =  5
    // Median  x˜ =  3
    // Mode  mode =  2, 3, 10
    // Standard Deviation  s =  4.3588989
    // Variance  s2 =  19
    static DS1: [i64; 3] = [2,3,10];

    #[test]
    fn test_dim_plot() {
        // create a plot with a width of three
        let mut pl = Plot::new(3, false, 1);
        assert_eq!(pl.width, 3);

        // add DS1 to plot
        let mut ds = ministat::DataSet::new("my_dataset");
        ds.add_points(&DS1);
        pl.dim_plot(&ds);

        // confirm the plot configuration
        assert_eq!(pl.min, 1.); // mean - floor(stddev) = 5 - 4 = 1
        assert_eq!(pl.max, 10.);
        assert_eq!(pl.span, 9.); // 10 - 9
        assert_eq!(pl.dx, 4.5); // span / (width - 1) = 9 / 2 = 4.5
        assert_eq!(pl.x0, -1.25); // min - .5*dx = 1 - 2.25 = -1.25

        // height remains zero because we have not yet plotted the dataset
        assert_eq!(pl.height, 0);

        // expect gen_buckets() == []
        // expect a hashmap == { 1:  }
        // expect plot(hashmap) == "..."
    }

    #[test]
    fn test_plot_set() {
        // create a plot with a width of three
        let mut pl = Plot::new(3, false, 1);
        assert_eq!(pl.width, 3);

        // add DS1 to plot
        let mut ds = ministat::DataSet::new("my_dataset");
        ds.add_points(&DS1);
        pl.plot_set(ds, 65);

        // confirm the plot configuration
        assert_eq!(pl.min, 1.); // mean - floor(stddev) = 5 - 4 = 1
        assert_eq!(pl.max, 10.);
        assert_eq!(pl.span, 9.); // 10 - 9
        assert_eq!(pl.dx, 4.5); // span / (width - 1) = 9 / 2 = 4.5
        assert_eq!(pl.x0, -1.25); // min - .5*dx = 1 - 2.25 = -1.25

        // given left edge is x0 = -1.25, and bucket size is dx = 4.5
        // buckets should be [-1.25, 3.25), [3.25, 7.75), [7.75, 12.25]
        // DS1 = {2,3,10}
        // 2 should go into first bucket, 3 should go into first bucket, 10 should go into last bucket
        // max height should therefore be 2, but plot_set adds 1 for buffer, so 3
        assert_eq!(pl.height, 3);

        // expect gen_buckets() == []
        // expect a hashmap == { 1:  }
        // expect plot(hashmap) == "..."
    }
}
