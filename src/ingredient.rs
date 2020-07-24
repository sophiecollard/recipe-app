pub struct Ingredient {
    id: u32,
    name: String,
}

impl Ingredient {

    fn new(id: u32, name: &str) -> Self {
        Ingredient {
            id,
            name: name.to_string()
        }
    }

}
