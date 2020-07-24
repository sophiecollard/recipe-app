use std::fmt;
use std::io;

pub enum Action {
    ListRecipesForIngredient,
    UpdateIngredients,
    UpdateRecipes,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = match self {
            Action::ListRecipesForIngredient => String::from("list recipes for an ingredient"),
            Action::UpdateIngredients => String::from("update ingredients"),
            Action::UpdateRecipes => String::from("update recipes"),
        };

        write!(f, "{}", s)
    }
}

pub fn prompt_for_action() -> Action {
    loop {
        println!("Please enter a number between 1 and 3 to choose from the following options:");
        println!("[1] List recipes for an ingredient");
        println!("[2] Update ingredients list");
        println!("[3] Update recipes");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let action: Action = match action.trim().parse() {
            Ok(1) => Action::ListRecipesForIngredient,
            Ok(2) => Action::UpdateIngredients,
            Ok(3) => Action::UpdateRecipes,
            _ => continue,
        };

        return action;
    }
}
