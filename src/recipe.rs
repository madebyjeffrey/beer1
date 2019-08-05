use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub ingredients: String,
    pub fermentables: Vec<Fermentable>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Fermentable {
    pub id: Uuid,
    pub amount: f64
}

pub struct FermentableDerived {
    pub amount: f64,
    pub name: String,
    pub manufacturer: String,
    pub cgai: f64,
    pub color: f64,
}
