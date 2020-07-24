mod action;

fn main() {
    println!("Welcome to the Recipe App!");

    let a = action::prompt_for_action();

    println!("You chose to {}", a);

    match a {
        action::Action::ListRecipesForIngredient => list_recipes(),
        action::Action::UpdateIngredients => update_ingredients(),
        action::Action::UpdateRecipes => update_recipes(),
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
