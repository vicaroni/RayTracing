use once_cell::unsync::Lazy;
use rand::{Rng, rngs::SmallRng, SeedableRng, distributions::{Standard, Uniform}, prelude::Distribution};

static mut RNG: Lazy<SmallRng> = Lazy::new(|| SmallRng::from_entropy());

pub fn gen<T>() -> T where Standard: Distribution<T>{
    unsafe {
        RNG.gen::<T>()
    }
}

pub fn sample(min: f64, max: f64) -> f64 {
    let dist = Uniform::new(min, max);
    unsafe {
        RNG.sample(dist)
    }
}
