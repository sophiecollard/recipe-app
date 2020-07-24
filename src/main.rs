mod action;

use crate::action::Action;

fn main() {
    println!("Welcome to the Recipe App!");

    let a = action::prompt_for_action();

    println!("You chose to {}", a);

    match a {
        Action::ListRecipesForIngredient => list_recipes(),
        Action::UpdateIngredients => update_ingredients(),
        Action::UpdateRecipes => update_recipes(),
    }
}

fn list_recipes() {
    println!("Not implemented!");
}

fn update_ingredients() {
    println!("Not implemented!");
}

fn update_recipes() {
    println!("Not implemented!");
}
