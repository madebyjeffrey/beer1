use std::option::Option::{Some, None};

pub fn single<T, I>(iterator: &mut I) -> Option<T>
    where I : Iterator<Item = T> {
        let once = iterator.next();
        let twice = iterator.next();

        match (once, twice) {
            (Some(x), None) => Some(x),
            _ => None
        }
    }