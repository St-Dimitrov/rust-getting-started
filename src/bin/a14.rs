// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn print_color(color: &str) {
    println!("Color: {:?}", color);
}

fn main() {
    let people = vec![
        Person {
            age: 9,
            name: String::from("Kiril"),
            favorite_color: "Blue".to_owned(),
        },
        Person {
            age: 18,
            name: String::from("Sasho"),
            favorite_color: "Grey".to_owned(),
        },
        Person {
            age: 10,
            name: String::from("Iliya"),
            favorite_color: String::from("Yellow"),
        }
    ];
    for someone in people {
        if someone.age <= 10 {
            print_name(&someone.name);
            print_color(&someone.favorite_color);
        }
    }
}
