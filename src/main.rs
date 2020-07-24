mod action;
mod ingredient;

use crate::action::Action;
use crate::ingredient::Ingredient;

fn main() {
    println!("Welcome to the Recipe App!");

    let mut ingredients: Vec<Ingredient> = Vec::new();

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
