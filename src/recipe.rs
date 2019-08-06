use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct Recipe {
    pub ingredients: String,
    pub fermentables: Vec<Fermentable>,
    pub liquor_to_grist: f64,
    pub mash_temp: f64,
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
