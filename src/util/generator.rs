use std::str::FromStr;

use chrono::NaiveDate;
use rand::{distr::Alphanumeric, Rng};
use rust_decimal::Decimal;

pub fn generate_numbers_decimal(min:i32, max:i32, scale: u32) -> Decimal {
    let mut rng = rand::rng();
    let r = format!("{}", rng.random_range(min..=max));
    let mut dynamic_value = Decimal::from_str(&r).unwrap();
    let _ = dynamic_value.set_scale(scale).unwrap();
    return  dynamic_value;
}

pub fn generate_numbers_i32(min:i32, max:i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..=max) 
}

pub fn generate_numbers_i64(min:i64, max:i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(min..=max) 
}

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

pub fn generate_naivedate(min_year: i32, max_year: i32) -> NaiveDate {
    let year = generate_numbers_i32(min_year, max_year);
    let date = NaiveDate::from_ymd_opt(year, 01, 01).unwrap();
    return date;
}