use rand::Rng;

pub fn random_seed() -> Result<u16, rand::Error> {
    let mut rng = rand::thread_rng();

    let random_number = rng.gen_range(1..=100);

    Ok(random_number)
}
