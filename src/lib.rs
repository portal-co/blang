use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha3::Digest;

/// Creates a seeded RNG from the provided arguments
pub fn create_seeded_rng(args: impl Iterator<Item = impl AsRef<[u8]>>) -> ChaCha20Rng {
    let mut hash = sha3::Sha3_256::default();
    for arg in args {
        hash.update(&arg);
        hash.update(&[0xff]);
    }
    ChaCha20Rng::from_seed(hash.finalize().into())
}

/// Generates a random string using the provided RNG
pub fn generate_random_string(rng: &mut ChaCha20Rng) -> String {
    (0..rng.random_range(2..4))
        .map(|_| match rng.random_range(0..3) {
            0 => rng.random_range(0xac00..(0xac00 + 11172)),
            1 => rng.random_range(0xa000..(0xa000 + 1165)),
            2 => rng.random_range(0x17000..(0x17000 + 6144)),
            _ => unreachable!(),
        })
        .filter_map(|a| char::from_u32(a))
        .collect::<String>()
}
