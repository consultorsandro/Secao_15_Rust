use std::{collections, io};

fn make_money(text: &mut String) {
    text.push_str("$$$");
}

fn trim_and_capitalize(text: &str) -> String {
    text.trim().to_uppercase()
}

fn elements(text: &str) -> Vec<&str> {
    text.split("!").collect::<Vec<&str>>()
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();
    println!("What's your first name?");
    input.read_line(&mut first_name).expect("Failed to read line");
    println!("What's your last name?");
    input.read_line(&mut last_name).expect("Failed to read line");
    // Using trim to remove any leading or trailing whitespace
    format!("{} {}", first_name.trim(), last_name.trim())   
}

fn main() {
    let mut amount = String::from("1000");
    make_money(&mut amount);
    println!("Amount with money: {}", amount);

    let banana = trim_and_capitalize("   banana   ");
    println!("Trimmed and capitalized: {}", banana);

    let collection = elements("Gold!Silver!Platinum!");
    println!("Elements: {:?}", collection);

    let full_name = get_identity();
    println!("Full name: {}", full_name);

}
/*
//Class 257
    let mut name = String::new();
    println!("Enter your name: ");
    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Hello, {}!", name.trim()),
        Err(message) => println!("Error: Algo errado aconteceu {}", message),
    };
*/
/*
//Class 255
    let mut music_genres = "         Rock, Metal, Country, Blues         ";
    println!("{}", music_genres.trim());
   
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    println!("{}", music_genres.replace("a", "@"));

    println!("{}", music_genres.trim());

    let genres: Vec<&str> = music_genres.split(",").collect();
    println!("{:?}", genres);

*/
/*
 //Class 253

    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let icon = format!("{} {}", first_name, last_name);

    println!("{}", icon);
*/

/*
    //&str
    //String

    let pirate = "Bloodhook";
    let sailer = String::from("Pirate");
    let bad_guy = pirate.to_string();

    println!("{} {} {}", pirate, sailer, bad_guy);

*/
