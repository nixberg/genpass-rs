fn main() {
    use rand::Rng;

    let security_level = parse_arguments();

    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let bits_per_charcter = (CHARSET.len() as f64).log2();
    let password_length = (security_level as f64 / bits_per_charcter).ceil() as usize;

    let mut rng = rand::thread_rng();

    let password: String = (0..password_length)
        .map(|_| {
            let index = rng.gen_range(0, CHARSET.len());
            CHARSET[index] as char
        })
        .collect();

    println!("{}", password);
}

fn parse_arguments() -> usize {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => Ok(128),
        2 => match args[1].as_ref() {
            "--help" | "-h" => print_help(),
            _ => args[1]
                .parse()
                .map_err(|_| format!("The value '{}' is invalid for '<security-level>'", args[1])),
        },
        3 => Err(format!("Unexpected argument '{}'", args[2])),
        n => Err(format!("{} unexpected arguments", n - 2)),
    }
    .unwrap_or_else(|error| {
        println!("Error: {}", error);
        println!("Usage: genpass [<security-level>]");
        std::process::exit(64);
    })
}

fn print_help() -> ! {
    println!(
        "USAGE: genpass [<security-level>]

ARGUMENTS:
  <security-level>        The desired security level. (default: 128)

OPTIONS:
  -h, --help              Show help information."
    );
    std::process::exit(0);
}
