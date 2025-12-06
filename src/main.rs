use blang::{create_seeded_rng, generate_random_string};

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let kind = args.next().unwrap();
    let (target, kind) = match kind.split_once("/") {
        Some(a) => a,
        None => (&*kind, ""),
    };
    let mut rng = create_seeded_rng(args);
    for l in std::io::stdin().lines() {
        let l = l?;
        let r = generate_random_string(&mut rng);
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
