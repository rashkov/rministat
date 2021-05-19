use ministat::*;
use std::env;
mod plot;

fn main() {
    // Read in file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ministat [-C column] [-c confidence] [-d delimiter(s)] [-ns] [-w width] [file [file ...]]");
        std::process::exit(1);
    }
    let datasets = args[1..]
        .iter()
        .map(|f| readset_mt(&f))
        .collect::<std::vec::Vec<_>>();
    datasets[0].vitals();
    for ds in &datasets[1..] {
        ds.vitals();
        ds.relative(&datasets[0], 2)
    }
    let mut plot = plot::Plot::new(100, false, datasets.len() as i32);
    for ds in &datasets {
        plot.dim_plot(&ds);
    }
}
