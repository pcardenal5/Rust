#[derive(Debug)]
struct Rectangle{
    width : u64,
    height : u64
}

fn area(rectangle : &Rectangle) -> u64 {
    rectangle.height * rectangle.width
}

fn main() {
    let rectangle = Rectangle{
        width : 10, 
        height: 5
    };

    println!("The area of {:#?} is {}", rectangle, area(&rectangle))
}
