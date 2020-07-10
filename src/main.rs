use std::io;

fn main() {
    println!("Welcome to the Recipe App!");

    let choice: u32 = prompt_for_option();

    println!("You chose option: {}", choice);
}

fn prompt_for_option() -> u32 {
    loop {
        println!("Please select one of the following options:");
        println!("[1] List recipes for an ingredient");
        println!("[2] Update ingredients list");
        println!("[3] Update recipes");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        return choice;
    }
}
