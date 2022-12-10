use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
};
use crypto_bigint::{NonZero, Random, Reciprocal, Uint};
use rand_core::OsRng;

fn bench_division<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    const TEST_SET: usize = 10;
    let xs = (0..TEST_SET)
        .map(|_| Uint::<4>::random(&mut OsRng))
        .collect::<Vec<_>>();
    let ys = (0..TEST_SET)
        .map(|_| Uint::<2>::ZERO.concat(&Uint::<2>::random(&mut OsRng)))
        .collect::<Vec<_>>();
    group.bench_function("div/rem, 4/2, full size", |b| {
        b.iter(|| {
            xs.iter()
                .zip(ys.iter())
                .map(|(x, y)| x.div_rem(&y))
                .for_each(drop)
        })
    });

    group.bench_function("rem, 4/2, full size", |b| {
        b.iter(|| {
            xs.iter()
                .zip(ys.iter())
                .map(|(x, y)| x.rem(&y))
                .for_each(drop)
        })
    });

    let ys = (0..TEST_SET)
        .map(|_| Uint::<1>::random(&mut OsRng))
        .collect::<Vec<_>>();
    let ys_full = ys
        .iter()
        .map(|y| Uint::<4>::from(y.as_limbs()[0]))
        .collect::<Vec<_>>();
    let ys_limb = ys
        .iter()
        .map(|y| NonZero::new(y.as_limbs()[0]).unwrap())
        .collect::<Vec<_>>();
    group.bench_function("div/rem, 4/1, full size", |b| {
        b.iter(|| {
            xs.iter()
                .zip(ys_full.iter())
                .map(|(x, y)| x.div_rem(&y))
                .for_each(drop)
        })
    });
    group.bench_function("div/rem, 4/1, single limb", |b| {
        b.iter(|| {
            xs.iter()
                .zip(ys_limb.iter())
                .map(|(x, y)| x.div_rem_limb(*y))
                .for_each(drop)
        })
    });

    let reciprocals = ys_limb
        .iter()
        .map(|y| Reciprocal::new(**y))
        .collect::<Vec<_>>();
    group.bench_function("div/rem, 4/1, single limb with reciprocal", |b| {
        b.iter(|| {
            xs.iter()
                .zip(reciprocals.iter())
                .map(|(x, r)| x.div_rem_limb_with_reciprocal(r))
                .for_each(drop)
        })
    });
}

fn bench_wrapping_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("wrapping ops");
    bench_division(&mut group);
    group.finish();
}

criterion_group!(benches, bench_wrapping_ops);
criterion_main!(benches);
