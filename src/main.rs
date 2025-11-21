use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha3::Digest;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let kind = args.next().unwrap();
    let (target, kind) = match kind.split_once("/") {
        Some(a) => a,
        None => (&*kind, ""),
    };
    let mut hash = sha3::Sha3_256::default();
    for arg in args {
        hash.update(&arg);
        hash.update(&[0xff]);
    }
    let mut rng = ChaCha20Rng::from_seed(hash.finalize().into());
    for l in std::io::stdin().lines() {
        let l = l?;
        let r = (0..rng.random_range(2..4))
            .map(|_| match rng.random_range(0..3) {
                0 => rng.random_range(0xac00..(0xac00 + 11172)),
                1 => rng.random_range(0xa000..(0xa000 + 1165)),
                2 => rng.random_range(0x17000..(0x17000 + 6144)),
                _ => todo!(),
            })
            .filter_map(|a| char::from_u32(a))
            .collect::<String>();
        match target {
            "ts:core" => {
                println!("export const k{}: string = \"{r}\";",l.chars().filter(|a|a.is_alphanumeric()).collect::<String>())
            }
            _ => {
                println!("{r} {l}");
            }
        }
    }
    Ok(())
}
