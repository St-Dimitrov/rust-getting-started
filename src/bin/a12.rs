// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum BoxColor {
    Brown,
    White,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Brown => println!("The color of the box is brown"),
            BoxColor::White => println!("The color of the box is white"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self){
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn new(weight: f64, color:BoxColor, dimensions:Dimensions) -> Self {
        Self {
            weight,
            color, 
            dimensions,
        }
    }

    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 20.0,
        height: 10.0,
        depth: 10.0
    };
    let small_box = ShippingBox::new(5.0, BoxColor::White, small_dimensions);
    small_box.print();
}
