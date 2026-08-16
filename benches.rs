//^
//^ HEAD
//^

//> HEAD -> SYSTEMSTD
use systemstd::{
    System,
    Read,
    Handling
};

//> HEAD -> CORE
use core::hint::black_box;

//> HEAD -> CRITERION
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
    Throughput
};


//^
//^ BENCHES
//^

//> BENCHES -> SETUP
criterion_group!(systemstd, benches);
criterion_main!(systemstd);

//> BENCHES -> RUN
fn benches(criterion: &mut Criterion) -> () {
    let mut group = criterion.benchmark_group("systemstd");
    const ITERATIONS: usize = 100;
    group.throughput(Throughput::Elements(ITERATIONS as u64));
    group.bench_function("print", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        System::print(black_box("hello!"), black_box(true));
    }));
    group.bench_function("openread", |bencher| bencher.iter(|| for _ in 0..ITERATIONS {
        System::expect::<System, _>(System::expect::<System, _>(
            System::path("README.md").file::<Read>(Handling::AssumeExists)
        ).read());
    }));
}