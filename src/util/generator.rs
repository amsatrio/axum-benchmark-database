use rand::{distr::Alphanumeric, Rng};

pub fn generate_numbers_f64(min:f64, max:f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..=max) 
}

pub fn generate_numbers_f32(min:f32, max:f32) -> f32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max) 
}

pub fn generate_numbers_usize(min:usize, max:usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(min..=max) 
}


pub fn generate_word(length: usize) -> String {
    let rng = rand::rng();

    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}