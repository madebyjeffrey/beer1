use std::fs::{read_to_string};
use std::path::Path;
use serde::de::{DeserializeOwned};
use serde_json::from_str;
use super::errors::FileErrors;

pub fn from_json<T>(source: &str) -> Result<T, FileErrors> 
    where T: DeserializeOwned {
    let path = Path::new(source);

    let contents = read_to_string(path).map_err(|_| FileErrors::UnableToReadFile)?;
    
    let object:T = from_str(&contents).map_err(|_| FileErrors::UnableToParseFile)?;
    
    Result::Ok(object)        
}