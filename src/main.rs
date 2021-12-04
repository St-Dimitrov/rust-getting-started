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
    whileloop()
}

// Repetitions

fn looping() {
    let mut z = 0;
    loop {
        if z == 5 {
            break
        }
        println!("{:?}", z);
        z = z + 1
    }
}

fn whileloop() {
    let mut y = 0;
    while y != 5{
        println!("{:?} {}", y, "with while");
        y = y + 1;
    }
}