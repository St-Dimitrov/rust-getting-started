// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let string_for_printing = "iNteReSting FORmaT".to_owned();
    let uppercase_string = string_for_printing.to_uppercase();
    let lowercase_string = string_for_printing.to_lowercase();
    println!("Uppercase is: {:?}", uppercase_string);
    println!("Lowercase is: {:?}", lowercase_string);
    println!("Both: {:?}, {:?}", string_for_printing.to_lowercase(), string_for_printing.to_uppercase())
}
