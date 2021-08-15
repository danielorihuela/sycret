use rand::Rng;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sycret::eq::*;
use sycret::stream::{FSSKey, PRG};
use sycret::utils::MMO;

pub fn eq_keygen(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let aes_keys: [u128; 4] = rng.gen();
    let mut prg = MMO::from_slice(&aes_keys);
    c.bench_function("Eq keygen", |b| {
        b.iter(|| EqKey::generate_keypair(black_box(&mut prg)))
    });
}

pub fn eq_eval(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let aes_keys: [u128; 4] = rng.gen();
    let mut prg = MMO::from_slice(&aes_keys);

    let (k_a, k_b) = EqKey::generate_keypair(&mut prg);
    let alpha = k_a.alpha_share.wrapping_add(k_b.alpha_share);

    c.bench_function("Eq eval", |b| {
        b.iter(|| k_a.eval(black_box(&mut prg), 0, alpha))
    });
}

criterion_group!(bench_eq, eq_keygen, eq_eval);
criterion_main!(bench_eq);
