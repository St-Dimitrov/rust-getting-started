fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = add(4, 6);
    if x > 9 {
        println!("Big number")
    } else {
        println!("Small number")
    }
    println!("{:?}", x);
}

