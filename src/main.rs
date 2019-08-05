mod ingredients;
mod session;
mod recipe;
mod errors;
mod json;
mod utility;

use ingredients::{Ingredients, lookup_malt, by_id};
use session::Session;
use recipe::{Recipe, FermentableDerived};
use json::from_json;
use utility::{single};

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

    let needed = recipe.fermentables
        .iter()
        .flat_map(|fermentable| {
            let first_option = single(&mut lookup_malt(&ingredients.grain, by_id(fermentable.id)))?;                

            Some(FermentableDerived {
                name: first_option.name.clone(),
                cgai: first_option.cgai,
                color: first_option.color,
                manufacturer: first_option.manufacturer.clone(),
                amount: fermentable.amount
            })
        })
        .collect::<Vec<_>>();

    // not quite done yet

    Result::Ok("Loaded everything.".to_string())
}
