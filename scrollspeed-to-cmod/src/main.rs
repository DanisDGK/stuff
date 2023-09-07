use std::io::{self, Write};

fn main() {
    let scroll_speed = read_input("Enter osu! mania scroll speed: ");
    let receptor_size = read_input("Enter Etterna receptor size (in %): ");
    let cmod = calculate_cmod(scroll_speed, receptor_size);
    println!("Etterna cmod: {cmod}")
}

fn read_input(prompt: &str) -> i32 {
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim().parse::<i32>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }
    }
}

fn calculate_cmod(scroll_speed: i32, receptor_size: i32) -> i32 {
    scroll_speed * 3200 / receptor_size
}
