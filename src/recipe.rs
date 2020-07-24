use crate::ingredient::Ingredient;
use crate::quantity::Quantity;

pub struct Recipe {
    id: u32,
    name: String,
    ingredients: Vec<(Ingredient, Quantity)>,
}

impl Recipe {

    fn new(id: u32, name: &str, ingredients: Vec<(Ingredient, Quantity)>) -> Self {
        Recipe {
            id,
            name: name.to_string(),
            ingredients
        }
    }

}
