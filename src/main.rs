mod ingredients;
mod session;
mod recipe;
mod errors;
mod json;

use ingredients::{Ingredients, lookup_malt, by_id};
use session::Session;
use recipe::{Recipe, FermentableDerived};
use json::from_json;
use Result::{Err, Ok};


fn main() {
    let session = run("session.json");

    print!("{}", match session {
        Err(why) => why,
        Ok(result) => result
    });

}

fn run(session: &str) -> Result<String, String> {
    let session: Session = from_json(session)
        .map_err(|why| format!("Unable to load brewing session: {}", why))?;
    
    let recipe: Recipe = from_json(&session.recipe)
        .map_err(|why| format!("Unable to load recipe. {}", why))?;

    let ingredients: Ingredients = from_json(&recipe.ingredients)
        .map_err(|why| format!("Unable to load ingredients. {}", why))?;

    let mut needed: Vec<FermentableDerived> = Vec::new();

    for fermentable in recipe.fermentables.iter() {
        let options = lookup_malt(&ingredients.grain, by_id(fermentable.id));

        if options.len() == 1 {
            needed.push(FermentableDerived {
                name: options[0].name.clone(),
                cgai: options[0].cgai,
                color: options[0].color,
                manufacturer: options[0].manufacturer.clone(),
                amount: fermentable.amount                 
            });
        }
    }

    // not quite done yet

    Result::Ok("Loaded everything.".to_string())
}
