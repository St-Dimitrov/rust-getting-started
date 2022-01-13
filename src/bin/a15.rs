// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(i32)
}

fn main() {
    let tickets = vec![
        Ticket::Backstage("Pe6o".to_owned(), 100.0),
        Ticket::Vip("Go6o".to_owned(), 50.0),
        Ticket::Standard(20)
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(holder_name, price) => println!("Backstage ticket Holder: {:?}, Price: {:?}", holder_name, price),
            Ticket::Vip(holder_name, price) => println!("Vip ticket Holder: {:?}, Price: {:?}", holder_name, price),
            Ticket::Standard(price) => println!("Standard ticket, Price: {:?}", price)
        }
    }
}
