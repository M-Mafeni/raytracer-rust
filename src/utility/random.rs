use rand::Rng;

// returns a double between 0 and 1
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max-min)*random_double()
}

