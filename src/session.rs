use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub fermenter_output: f64,
    pub bhefficient: f64,
    pub target: f64,
    pub recipe: String,
}