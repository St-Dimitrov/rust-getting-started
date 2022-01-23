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
    pub fn TimeNow() {
        println!("{:?}", chrono::offset::Local::now());
        println!("{:?}", chrono::offset::Utc::now());
    }
}

fn main() {
    time::TimeNow();
}
