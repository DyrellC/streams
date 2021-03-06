#[macro_use]
extern crate criterion;

use criterion::{
    Benchmark,
    Criterion,
};
use iota_streams_core::{
    prng::Prng,
    sponge::spongos::Spongos,
    tbits::{
        trinary::Trit,
        Tbits,
    },
};
use iota_streams_core_keccak::sponge::prp::keccak::KeccakF1600T;
use iota_streams_core_ntru::key_encapsulation::ntru;
use std::{
    str::FromStr,
    time::Duration,
};

fn ntru_troika_benchmark(c: &mut Criterion) {
    let duration_ms = 1000;

    {
        let prngk = Tbits::<Trit>::cycle_str(Prng::<Trit, KeccakF1600T>::KEY_SIZE, "PRNGK");
        let prng = Prng::<Trit, KeccakF1600T>::init(prngk);
        let nonce = Tbits::<Trit>::from_str("NONCE").unwrap();

        c.bench(
            "Run NTRU KeccakF1600T",
            Benchmark::new("keygen", move |b| {
                b.iter(|| {
                    ntru::gen_keypair::<_, KeccakF1600T, _>(&prng, nonce.slice());
                })
            })
            .sample_size(10)
            .measurement_time(Duration::from_millis(duration_ms)),
        );
    }

    {
        let prngk = Tbits::<Trit>::from_str("PRNGK").unwrap();
        let prng = Prng::<Trit, KeccakF1600T>::init(prngk);
        let nonce = Tbits::<Trit>::from_str("NONCE").unwrap();

        let key = {
            let mut s = Spongos::<Trit, KeccakF1600T>::init();
            s.commit();
            s.squeeze_tbits(Spongos::<Trit, KeccakF1600T>::KEY_SIZE)
        };
        let key_size = key.size();

        let (sk, pk) = ntru::gen_keypair::<_, KeccakF1600T, _>(&prng, nonce.slice());
        let mut ekey = Tbits::<Trit>::zero(ntru::EKEY_SIZE);
        {
            let mut s = Spongos::<Trit, KeccakF1600T>::init();
            pk.encrypt_with_spongos(&mut s, &prng, nonce.slice(), key.slice(), ekey.slice_mut());
        }

        {
            let mut ekey2 = Tbits::<Trit>::zero(ntru::EKEY_SIZE);
            c.bench(
                "Run NTRU KeccakF1600T",
                Benchmark::new("encrypt", move |b| {
                    b.iter(|| {
                        let mut s = Spongos::<Trit, KeccakF1600T>::init();
                        pk.encrypt_with_spongos(&mut s, &prng, nonce.slice(), key.slice(), ekey2.slice_mut());
                    })
                })
                .sample_size(10)
                .measurement_time(Duration::from_millis(duration_ms)),
            );
        }

        {
            let mut key2 = Tbits::<Trit>::zero(key_size);
            c.bench(
                "Run NTRU KeccakF1600T",
                Benchmark::new("decrypt", move |b| {
                    b.iter(|| {
                        let mut s = Spongos::<Trit, KeccakF1600T>::init();
                        sk.decrypt_with_spongos(&mut s, ekey.slice(), key2.slice_mut());
                    })
                })
                .sample_size(10)
                .measurement_time(Duration::from_millis(duration_ms)),
            );
        }
    }
}

criterion_group!(benches, ntru_troika_benchmark);
criterion_main!(benches);
