use blang::{create_seeded_rng, generate_random_string, Target};

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let kind = args.next().unwrap();
    let (target_str, _kind) = match kind.split_once("/") {
        Some(a) => a,
        None => (&*kind, ""),
    };
    let target = Target::from_str(target_str);
    let mut rng = create_seeded_rng(args);
    for l in std::io::stdin().lines() {
        let l = l?;
        let r = generate_random_string(&mut rng);
        target.print(&l, &r);
    }
    Ok(())
}
