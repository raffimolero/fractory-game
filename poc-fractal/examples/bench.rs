use rand::{distributions::Uniform, prelude::*};
use std::{
    collections::{BTreeSet, HashSet},
    iter::repeat_with,
    time::Instant,
};

fn bench(name: &str, f: impl FnOnce()) {
    let start = Instant::now();
    f();
    let end = Instant::now();
    println!("{name} took {:?}", end - start);
}

fn main() {
    let data = Uniform::new_inclusive(0, u16::MAX)
        .sample_iter(thread_rng())
        .take(1 << 18)
        .collect::<Vec<_>>();

    let mut hash = HashSet::new();
    let mut btree = BTreeSet::new();

    bench("hash", || {
        for val in &data {
            hash.insert(val);
        }
    });
    bench("btree", || {
        for val in &data {
            btree.insert(val);
        }
    });
}
