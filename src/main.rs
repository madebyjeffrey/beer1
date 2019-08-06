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
use utility::{single, sg_to_plato};

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

    let required_ingredients = recipe.fermentables
        .iter()
        .flat_map(|fermentable| {
            let first_option = single(lookup_malt(&ingredients.grain, by_id(fermentable.id)))?;                

            Some(FermentableDerived {
                name: first_option.name.clone(),
                cgai: first_option.cgai,
                color: first_option.color,
                manufacturer: first_option.manufacturer.clone(),
                amount: fermentable.amount
            })
        })
        .collect::<Vec<_>>();

    println!("Total volume to fermenter: {} L", session.fermenter_output);
    println!("Brewhouse Efficiency: {} %", session.bhefficient);
    println!("Target: S.G. {:.3}, {:.1}\u{00B0}P", session.target, sg_to_plato(session.target));

    let net_extract = session.fermenter_output * session.target * sg_to_plato(session.target) / 100.0;

    println!("Net Extract is {:.2} kg", net_extract);

    let gross_extract = net_extract / (session.bhefficient / 100.0);

    println!("Gross Extract is {:.2} kg", gross_extract);
    println!();
    println!("Ingredients");
    println!("{:=<60}", "");
    
    for ingredient in required_ingredients.iter() {
        let amt = gross_extract * (ingredient.amount / 100.0) / (ingredient.cgai / 100.0);

        println!("{:30} {:20.2} kg", &ingredient.name, amt);
    }

    println!("");
    println!("Supplemental Information");

    let color = required_ingredients
        .iter()
        .map(|ingredient| (ingredient.amount / 100.0) * (ingredient.color) * (sg_to_plato(session.target) / 8.0))
        .fold(0.0, |sum, color| sum + color);

    println!("Color of wort: {:.1} SRM", color);
    println!("Color of beer: {:.1} SRM", color * 0.7);

    let strike = (0.4 * (recipe.mash_temp - session.malt_temp)) / recipe.liquor_to_grist + recipe.mash_temp;
    println!("Target Mash Temp: {:.1}\u{00B0}C", recipe.mash_temp);
    println!("Malt Temp: {:.1}\u{00B0}C", session.malt_temp);
    println!("Strike Temp: {:.1}\u{00B0}C", strike);


    println!("");

    Result::Ok("Done.".to_string())
}
