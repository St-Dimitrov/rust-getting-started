// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

mod time {
    use chrono;
    pub fn time_now() {
        println!("{:?}", chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
    }
}

fn main() {
    time::time_now();
}
