use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
    SamplingMode, Throughput,
};

const INPUT_DATA: [i32; 21] = [
    128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576,
    2097152, 4194304, 8388608, 16777216, 33554432, 67108864, 134217728,
];

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum");
    group.sampling_mode(SamplingMode::Flat);
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for i in INPUT_DATA.iter() {
        group.throughput(Throughput::Elements(*i as u64));
        let path = ["./test/data/", &i.to_string()[..], ".txt"].join("");
        group.bench_function(BenchmarkId::new("Buffered", i), |b| {
            b.iter(|| ministat::buf_read_sum(&path))
        });
        group.bench_function(BenchmarkId::new("Single-threaded", i), |b| {
            b.iter(|| ministat::read_to_str_sum(&path))
        });
        group.bench_function(BenchmarkId::new("Multi-threaded", i), |b| {
            b.iter(|| ministat::mt_sum(&path))
        });
    }
    group.finish();
}

fn bench_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort");
    group.sampling_mode(SamplingMode::Flat);
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for i in INPUT_DATA.iter() {
        group.throughput(Throughput::Elements(*i as u64));
        let path = ["./test/data/", &i.to_string()[..], ".txt"].join("");
        group.bench_function(BenchmarkId::new("Buffered", i), |b| {
            b.iter(|| ministat::buf_read_sorted(&path))
        });
        group.bench_function(BenchmarkId::new("Single-threaded", i), |b| {
            b.iter(|| ministat::read_to_str_sorted(&path))
        });
        group.bench_function(BenchmarkId::new("Multi-threaded", i), |b| {
            b.iter(|| ministat::mt_sorted(&path))
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_sum, bench_sort
}

criterion_main!(benches);
