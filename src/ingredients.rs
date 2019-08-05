use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Malt {
    pub name: String,
    pub manufacturer: String,
    pub cgai: f64,
    pub color: f64,
    pub moisture: f64,
    pub source: String,
    pub uuid: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct Ingredients {
    pub grain: Vec<Malt>,
}

pub fn by_id(id: Uuid) -> impl Fn(&Malt) -> bool {
    move |malt| malt.uuid == id 
}

pub fn lookup_malt<F: Fn(&Malt) -> bool>(grain: &Vec<Malt>, pred: F) -> Vec<Malt> {
    let mut collection = Vec::new();

    for malt in grain.iter() {
        if (pred(malt)) {
            collection.push(malt.clone());
        }
    }

    collection
}
// impl Ingredients {
//     pub fn load(source: &str) -> Result<Ingredients, FileErrors> {
//         from_json::<IngredientsDB>(source).map(|db| Ingredients { db })        
//     }

//     pub fn malts<U>(&self, pred: fn(&Malt) -> bool) -> Vec<Malt> {
//         let mut collection = Vec::new();

//         for malt in self.db.grain.iter() {
//             if pred(malt) {
//                 collection.push(malt.clone())
//             }
//         }

//         collection
//     }
// }