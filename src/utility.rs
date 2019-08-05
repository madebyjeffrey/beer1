use std::option::Option::{Some, None};
use num::pow;

pub fn single<T, I>(iterator: &mut I) -> Option<T>
    where I : Iterator<Item = T> {
        let once = iterator.next();
        let twice = iterator.next();

        match (once, twice) {
            (Some(x), None) => Some(x),
            _ => None
        }
    }

pub fn sg_to_plato(sg: f64) -> f64 {
     (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * pow(sg, 2)) + (135.997 * pow(sg, 3))
}