fn main() {
    use rand::Rng;

    const PASSWORD_LEN: usize = 25;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let index = rng.gen_range(0, CHARSET.len());
            CHARSET[index] as char
        })
        .collect();

    println!("{}", password);
}
