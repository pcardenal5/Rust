fn area(dimensions :(u64, u64)) -> u64 {
    dimensions.0 * dimensions.1
}

fn main() {
    let dimensions = (10,5);
    println!("The area of a rectangle of dimensions {}x{} is {}", dimensions.0, dimensions.1, area(dimensions))
}
